use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn native_console_log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = error)]
    pub fn native_console_error(s: &str);
}

#[allow(unused_macros)]
macro_rules! trace {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (console::native_console_log(&("[TRACE] ".to_owned() + &format_args!($($t)*).to_string())))
}

macro_rules! log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (console::native_console_log(&format_args!($($t)*).to_string()))
}

macro_rules! error {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (console::native_console_error(&format_args!($($t)*).to_string()))
}

pub(crate) use log;
pub(crate) use error;

#[allow(unused_imports)]
pub(crate) use trace;