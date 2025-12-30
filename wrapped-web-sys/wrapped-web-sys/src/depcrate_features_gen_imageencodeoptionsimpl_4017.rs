// Generated macro for impl_4017 (impl)
macro_rules! Depcrate_features_gen_ImageEncodeOptionsimpl_4017 {
() => {
// Module: crate::features::gen_ImageEncodeOptions
// Provides: {"impl_4017"}
// Dependencies: {}
impl ImageEncodeOptions { # [doc = "Construct a new `ImageEncodeOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ImageEncodeOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_quality()` instead."] pub fn quality (& mut self , val : f64) -> & mut Self { self . set_quality (val) ; self } # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : & str) -> & mut Self { self . set_type (val) ; self } }
};
}
