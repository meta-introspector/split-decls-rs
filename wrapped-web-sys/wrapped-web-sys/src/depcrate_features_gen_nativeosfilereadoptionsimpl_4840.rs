// Generated macro for impl_4840 (impl)
macro_rules! Depcrate_features_gen_NativeOsFileReadOptionsimpl_4840 {
() => {
// Module: crate::features::gen_NativeOsFileReadOptions
// Provides: {"impl_4840"}
// Dependencies: {}
impl NativeOsFileReadOptions { # [doc = "Construct a new `NativeOsFileReadOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bytes()` instead."] pub fn bytes (& mut self , val : Option < f64 >) -> & mut Self { self . set_bytes (val) ; self } # [deprecated = "Use `set_encoding()` instead."] pub fn encoding (& mut self , val : Option < & str >) -> & mut Self { self . set_encoding (val) ; self } }
};
}
