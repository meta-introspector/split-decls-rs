// Generated macro for impl_7749 (impl)
macro_rules! Depcrate_features_gen_TextDecodeOptionsimpl_7749 {
() => {
// Module: crate::features::gen_TextDecodeOptions
// Provides: {"impl_7749"}
// Dependencies: {}
impl TextDecodeOptions { # [doc = "Construct a new `TextDecodeOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `TextDecodeOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_stream()` instead."] pub fn stream (& mut self , val : bool) -> & mut Self { self . set_stream (val) ; self } }
};
}
