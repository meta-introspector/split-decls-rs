// Generated macro for impl_4415 (impl)
macro_rules! Depcrate_features_gen_MediaKeysPolicyimpl_4415 {
() => {
// Module: crate::features::gen_MediaKeysPolicy
// Provides: {"impl_4415"}
// Dependencies: {}
impl MediaKeysPolicy { # [doc = "Construct a new `MediaKeysPolicy`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `MediaKeysPolicy`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_min_hdcp_version()` instead."] pub fn min_hdcp_version (& mut self , val : & str) -> & mut Self { self . set_min_hdcp_version (val) ; self } }
};
}
