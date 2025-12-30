// Generated macro for impl_5797 (impl)
macro_rules! Depcrate_features_gen_ReadableStreamReadResultimpl_5797 {
() => {
// Module: crate::features::gen_ReadableStreamReadResult
// Provides: {"impl_5797"}
// Dependencies: {}
impl ReadableStreamReadResult { # [doc = "Construct a new `ReadableStreamReadResult`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ReadableStreamReadResult`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_done()` instead."] pub fn done (& mut self , val : bool) -> & mut Self { self . set_done (val) ; self } # [deprecated = "Use `set_value()` instead."] pub fn value (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_value (val) ; self } }
};
}
