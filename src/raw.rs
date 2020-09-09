/// This module provides raw access to the `fetch` API. WARNING: Here be dragons.

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/fetch_ffi.js")]
extern "C" {
    pub async fn fetch_json(
        url: &str,
        options: Option<js_sys::Object>
    ) -> JsValue;

    pub async fn fetch_text(
        url: &str,
        options: Option<js_sys::Object>
    ) -> JsValue;
}

/// A macro to create a new JS object.
#[macro_export]
macro_rules! object {
    ($($property:expr => $value:expr),* $(,)?) => { unsafe {
        let obj = js_sys::Object::new();
        $(
            match js_sys::Reflect::set(&obj, &$property.into(), &$value.into()) {
                Ok(_) => {/*Don't care if `$property` was already present*/},
                Err(err) => return Err(err),
            }
        )*
        obj
    }}
}
