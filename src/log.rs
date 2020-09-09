/// This module access to JavaScript's `console.log()` and `console.error()`
/// functionality.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = "log")]
    pub fn info(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = "error")]
    pub fn error(s: &str);
}

/// Write a message using `console.log()`.
#[macro_export]
macro_rules! info {
    ($fmt:expr $(, $arg:expr)*) => {
        #[allow(unused_unsafe)]
        unsafe { crate::log::info(&format!($fmt $(, $arg)*)) }
    };
}

/// Write a message using `console.error()`.
#[macro_export]
macro_rules! error {
    ($fmt:expr $(, $arg:expr)*) => {
        #[allow(unused_unsafe)]
        unsafe { crate::log::error(&format!($fmt $(, $arg)*)) }
    };
}
