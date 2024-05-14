mod utils;

use gloo_utils::format::JsValueSerdeExt;
use indexmap::IndexMap;
use js_sys::{Array, Object, Reflect};
use serde_json_bytes::serde_json::value::Index;
use wasm_bindgen::prelude::*;

use apollo_federation::sources::connect::{ApplyTo, JSONSelection};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-bridge!");
}

#[wasm_bindgen]
pub fn parse_selection_and_apply_to(selection: &str, value: String, vars: String) -> JsValue {
    let value = match serde_json_bytes::Value::from_bytes(value.into()) {
        Ok(value) => value,
        Err(err) => {
            let obj = Object::new();
            let errs = Array::new();
            Reflect::set(
                &errs,
                &0.into(),
                &JsValue::from_serde(&err.to_string()).unwrap(),
            )
            .unwrap();
            Reflect::set(&obj, &"errors".into(), &errs).unwrap();
            return obj.into();
        }
    };

    let vars = match serde_json_bytes::Value::from_bytes(vars.into()) {
        Ok(value) => value
            .as_object()
            .map(|obj| {
                obj.into_iter()
                    .map(|(k, v)| (k.as_str().to_owned(), v.clone()))
                    .collect()
            })
            .unwrap_or_default(),
        Err(err) => {
            let obj = Object::new();
            let errs = Array::new();
            Reflect::set(
                &errs,
                &0.into(),
                &JsValue::from_serde(&err.to_string()).unwrap(),
            )
            .unwrap();
            Reflect::set(&obj, &"errors".into(), &errs).unwrap();
            return obj.into();
        }
    };

    let selection = match JSONSelection::parse(selection) {
        Ok(selection) => selection,
        Err(err) => {
            let obj = Object::new();
            let errs = Array::new();
            Reflect::set(
                &errs,
                &0.into(),
                &JsValue::from_serde(&err.to_string()).unwrap(),
            )
            .unwrap();
            Reflect::set(&obj, &"errors".into(), &errs).unwrap();
            return obj.into();
        }
    };

    let (value, errors) = selection.1.apply_with_vars(&value, &vars);

    let obj = Object::new();
    Reflect::set(&obj, &"value".into(), &JsValue::from_serde(&value).unwrap()).unwrap();

    let errs = Array::new();
    for (i, err) in errors.iter().enumerate() {
        Reflect::set(
            &errs,
            &i.into(),
            &JsValue::from_serde(err.message().unwrap_or("unknown error")).unwrap(),
        )
        .unwrap();
    }
    Reflect::set(&obj, &"errors".into(), &errs).unwrap();
    obj.into()
}
