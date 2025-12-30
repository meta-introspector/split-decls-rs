// Generated macro for impl_5790 (impl)
macro_rules! Depcrate_features_gen_ReadableStreamIteratorOptionsimpl_5790 {
() => {
// Module: crate::features::gen_ReadableStreamIteratorOptions
// Provides: {"impl_5790"}
// Dependencies: {}
impl ReadableStreamIteratorOptions { # [doc = "Construct a new `ReadableStreamIteratorOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ReadableStreamIteratorOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_prevent_cancel()` instead."] pub fn prevent_cancel (& mut self , val : bool) -> & mut Self { self . set_prevent_cancel (val) ; self } }
};
}
