mod utils;

use apollo_compiler::collections::IndexMap;
use apollo_federation::sources::connect::{ApplyTo, JSONSelection};
use gloo_utils::format::JsValueSerdeExt;
use js_sys::{Array, Object, Reflect};
use wasm_bindgen::prelude::*;

// use apollo_federation::sources::connect::json_selection::NamedSelection;
// use apollo_federation::sources::connect::{
//     ApplyTo, JSONSelection, Key, PathSelection, SubSelection,
// };

// fn to_ast(selection: &JSONSelection) -> JsValue {
//     match selection {
//         JSONSelection::Named(sub_selection) => sub_selection_to_ast(sub_selection),
//         JSONSelection::Path(path_selection) => path_selection_to_ast(path_selection),
//     }
// }

// fn path_selection_to_ast(path_selection: &PathSelection) -> JsValue {
//     let obj = Object::new();
//     match path_selection {
//         PathSelection::Var(name, selections) => {
//             Reflect::set(&obj, &"kind".into(), &"Path(Var)".into()).unwrap();
//             Reflect::set(&obj, &"name".into(), &name.as_str().into()).unwrap();
//             Reflect::set(
//                 &obj,
//                 &"selections".into(),
//                 &path_selection_to_ast(selections),
//             )
//             .unwrap();
//         }
//         PathSelection::Key(key, selections) => {
//             Reflect::set(&obj, &"kind".into(), &"Path(Key)".into()).unwrap();
//             Reflect::set(&obj, &"key".into(), &key_to_ast(key)).unwrap();
//             Reflect::set(
//                 &obj,
//                 &"selections".into(),
//                 &path_selection_to_ast(selections),
//             )
//             .unwrap();
//         }
//         PathSelection::Selection(sub_selection) => {
//             Reflect::set(&obj, &"kind".into(), &"Path(Selection)".into()).unwrap();
//             Reflect::set(
//                 &obj,
//                 &"selections".into(),
//                 &sub_selection_to_ast(sub_selection),
//             )
//             .unwrap();
//         }
//         PathSelection::Empty => {
//             Reflect::set(&obj, &"kind".into(), &"Path(Empty)".into()).unwrap();
//         }
//     }
//     obj.into()
// }

// fn key_to_ast(key: &Key) -> JsValue {
//     let obj = Object::new();
//     match key {
//         Key::Field(field) => {
//             Reflect::set(&obj, &"field".into(), &field.as_str().into()).unwrap();
//         }
//         Key::Quoted(quoted) => {
//             Reflect::set(&obj, &"quoted".into(), &quoted.as_str().into()).unwrap();
//         }
//         Key::Index(index) => {
//             Reflect::set(&obj, &"index".into(), &index.to_string().as_str().into()).unwrap();
//         }
//     }
//     obj.into()
// }

// fn sub_selection_to_ast(sub_selection: &SubSelection) -> JsValue {
//     let obj = Array::new();
//     for selection in sub_selection.selections.iter() {
//         obj.push(&named_selection_to_ast(selection));
//     }
//     if let Some(star) = &sub_selection.star {
//         let star_obj = Object::new();
//         Reflect::set(&star_obj, &"kind".into(), &"Star".into()).unwrap();
//         if let Some(alias) = &star.0 {
//             Reflect::set(&star_obj, &"alias".into(), &alias.name.as_str().into()).unwrap();
//         }
//         if let Some(sub_selection) = &star.1 {
//             Reflect::set(
//                 &star_obj,
//                 &"selections".into(),
//                 &sub_selection_to_ast(sub_selection),
//             )
//             .unwrap();
//         }
//         obj.push(&star_obj.into());
//     }
//     obj.into()
// }

// fn named_selection_to_ast(named_selection: &NamedSelection) -> JsValue {
//     let obj = Object::new();
//     match named_selection {
//         NamedSelection::Field(alias, name, sub_selection) => {
//             Reflect::set(&obj, &"kind".into(), &"Named(Field)".into()).unwrap();
//             if let Some(alias) = alias {
//                 Reflect::set(&obj, &"alias".into(), &alias.name.as_str().into()).unwrap();
//             }
//             Reflect::set(&obj, &"name".into(), &name.as_str().into()).unwrap();
//             if let Some(sub_selection) = sub_selection {
//                 Reflect::set(
//                     &obj,
//                     &"selections".into(),
//                     &sub_selection_to_ast(sub_selection),
//                 )
//                 .unwrap();
//             }
//         }
//         NamedSelection::Quoted(alias, name, sub_selection) => {
//             Reflect::set(&obj, &"kind".into(), &"Named(Quoted)".into()).unwrap();
//             Reflect::set(&obj, &"alias".into(), &alias.name.as_str().into()).unwrap();
//             Reflect::set(&obj, &"name".into(), &name.as_str().into()).unwrap();
//             if let Some(sub_selection) = sub_selection {
//                 Reflect::set(
//                     &obj,
//                     &"selections".into(),
//                     &sub_selection_to_ast(sub_selection),
//                 )
//                 .unwrap();
//             }
//         }
//         NamedSelection::Path(alias, path_selection) => {
//             Reflect::set(&obj, &"kind".into(), &"Named(Path)".into()).unwrap();
//             Reflect::set(&obj, &"alias".into(), &alias.name.as_str().into()).unwrap();
//             Reflect::set(
//                 &obj,
//                 &"selections".into(),
//                 &path_selection_to_ast(path_selection),
//             )
//             .unwrap();
//         }
//         NamedSelection::Group(alias, sub_selection) => {
//             Reflect::set(&obj, &"kind".into(), &"Named(Group)".into()).unwrap();
//             Reflect::set(&obj, &"alias".into(), &alias.name.as_str().into()).unwrap();
//             Reflect::set(
//                 &obj,
//                 &"selections".into(),
//                 &sub_selection_to_ast(sub_selection),
//             )
//             .unwrap();
//         }
//     }
//     obj.into()
// }

// #[wasm_bindgen]
// pub fn parse_selection(selection: &str) -> JsValue {
//     match JSONSelection::parse(selection) {
//         Ok(selection) => to_ast(&selection.1),
//         Err(err) => {
//             let obj = Object::new();
//             let errs = Array::new();
//             Reflect::set(
//                 &errs,
//                 &0.into(),
//                 &JsValue::from_serde(&err.to_string()).unwrap(),
//             )
//             .unwrap();
//             Reflect::set(&obj, &"errors".into(), &errs).unwrap();
//             return obj.into();
//         }
//     }
// }

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

    let vars: IndexMap<_, _> = match serde_json_bytes::Value::from_bytes(vars.into()) {
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

    let vars = vars.iter().map(|(k, v)| (k.clone(), v)).collect();

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
