// Generated macro for exit (function)
macro_rules! Depcrateexit {
() => {
// Module: crate
// Provides: {"exit"}
// Dependencies: {}
# [doc = " Used for debugging to the console"] pub fn exit (message : & str) -> ! { let v = wasm_bindgen :: JsValue :: from_str (message) ; web_sys :: console :: exception_1 (& v) ; std :: process :: abort () }
};
}
