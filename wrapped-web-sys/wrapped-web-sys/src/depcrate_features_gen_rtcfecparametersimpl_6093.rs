// Generated macro for impl_6093 (impl)
macro_rules! Depcrate_features_gen_RtcFecParametersimpl_6093 {
() => {
// Module: crate::features::gen_RtcFecParameters
// Provides: {"impl_6093"}
// Dependencies: {}
impl RtcFecParameters { # [doc = "Construct a new `RtcFecParameters`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcFecParameters`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_ssrc()` instead."] pub fn ssrc (& mut self , val : u32) -> & mut Self { self . set_ssrc (val) ; self } }
};
}
