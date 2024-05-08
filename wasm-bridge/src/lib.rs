mod utils;

use gloo_utils::format::JsValueSerdeExt;
use js_sys::{Array, Object, Reflect};
use wasm_bindgen::prelude::*;

use apollo_federation::sources::connect::{ApplyTo, Selection};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-bridge!");
}

#[wasm_bindgen]
pub fn parse_selection_and_apply_to(selection: &str, value: String) -> JsValue {
    let value = match serde_json_bytes::Value::from_bytes(value.into()) {
        Ok(value) => value,
        Err(err) => {
            let obj = Object::new();
            Reflect::set(
                &obj,
                &"errors".into(),
                &JsValue::from_serde(&err.to_string()).unwrap(),
            )
            .unwrap();
            return obj.into();
        }
    };

    let selection = match Selection::parse(selection) {
        Ok(selection) => selection,
        Err(err) => {
            let obj = Object::new();
            Reflect::set(
                &obj,
                &"errors".into(),
                &JsValue::from_serde(&err.to_string()).unwrap(),
            )
            .unwrap();
            return obj.into();
        }
    };

    let (value, errors) = selection.1.apply_to(&value);

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
