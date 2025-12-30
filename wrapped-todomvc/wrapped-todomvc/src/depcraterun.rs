// Generated macro for run (function)
macro_rules! Depcraterun {
() => {
// Module: crate
// Provides: {"run"}
// Dependencies: {}
# [doc = " Entry point into the program from JavaScript"] # [wasm_bindgen (start)] fn run () -> Result < () , JsValue > { console_error_panic_hook :: set_once () ; app ("todos-wasmbindgen") ; Ok (()) }
};
}
