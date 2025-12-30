// Generated macro for impl_4399 (impl)
macro_rules! Depcrate_features_gen_MediaKeySystemMediaCapabilityimpl_4399 {
() => {
// Module: crate::features::gen_MediaKeySystemMediaCapability
// Provides: {"impl_4399"}
// Dependencies: {}
impl MediaKeySystemMediaCapability { # [doc = "Construct a new `MediaKeySystemMediaCapability`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_content_type()` instead."] pub fn content_type (& mut self , val : & str) -> & mut Self { self . set_content_type (val) ; self } # [deprecated = "Use `set_robustness()` instead."] pub fn robustness (& mut self , val : & str) -> & mut Self { self . set_robustness (val) ; self } }
};
}
