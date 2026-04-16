use js_sys::{Object, Reflect};
use wasm_bindgen::prelude::*;

/// Run the full RustViz pipeline in the browser. Returns a JS object `{ vis_code, vis_timeline }` (SVG strings).
#[wasm_bindgen]
pub fn render_rustviz_wasm(
    main_rs: &str,
    annotated_source_rs: &str,
    source_rs: &str,
) -> Result<JsValue, JsValue> {
    match rustviz_lib::render_rustviz_from_strings(
        main_rs,
        annotated_source_rs,
        source_rs,
        "playground",
    ) {
        Ok((vis_code, vis_timeline)) => {
            let obj = Object::new();
            Reflect::set(&obj, &JsValue::from_str("vis_code"), &JsValue::from_str(&vis_code))
                .map_err(|_| JsValue::from_str("Reflect::set vis_code failed"))?;
            Reflect::set(
                &obj,
                &JsValue::from_str("vis_timeline"),
                &JsValue::from_str(&vis_timeline),
            )
            .map_err(|_| JsValue::from_str("Reflect::set vis_timeline failed"))?;
            Ok(obj.into())
        }
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}

/// Run source-only RustViz inference in the browser from one Rust input.
/// Returns a JS object `{ vis_code, vis_timeline }` (SVG strings).
#[wasm_bindgen]
pub fn render_rustviz_source_wasm(source_rs: &str) -> Result<JsValue, JsValue> {
    match rustviz_lib::render_rustviz_from_source(source_rs, "playground") {
        Ok((vis_code, vis_timeline)) => {
            let obj = Object::new();
            Reflect::set(&obj, &JsValue::from_str("vis_code"), &JsValue::from_str(&vis_code))
                .map_err(|_| JsValue::from_str("Reflect::set vis_code failed"))?;
            Reflect::set(
                &obj,
                &JsValue::from_str("vis_timeline"),
                &JsValue::from_str(&vis_timeline),
            )
            .map_err(|_| JsValue::from_str("Reflect::set vis_timeline failed"))?;
            Ok(obj.into())
        }
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}

/// Same pipeline as [`render_rustviz_source_wasm`] but returns `{ vis_combined }` (single SVG).
#[wasm_bindgen]
pub fn render_rustviz_source_combined_wasm(source_rs: &str) -> Result<JsValue, JsValue> {
    match rustviz_lib::render_rustviz_from_source_combined(source_rs, "playground") {
        Ok(vis_combined) => {
            let obj = Object::new();
            Reflect::set(
                &obj,
                &JsValue::from_str("vis_combined"),
                &JsValue::from_str(&vis_combined),
            )
            .map_err(|_| JsValue::from_str("Reflect::set vis_combined failed"))?;
            Ok(obj.into())
        }
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}
