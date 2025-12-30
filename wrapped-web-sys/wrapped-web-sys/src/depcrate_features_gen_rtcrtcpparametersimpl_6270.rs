// Generated macro for impl_6270 (impl)
macro_rules! Depcrate_features_gen_RtcRtcpParametersimpl_6270 {
() => {
// Module: crate::features::gen_RtcRtcpParameters
// Provides: {"impl_6270"}
// Dependencies: {}
impl RtcRtcpParameters { # [doc = "Construct a new `RtcRtcpParameters`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcRtcpParameters`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_cname()` instead."] pub fn cname (& mut self , val : & str) -> & mut Self { self . set_cname (val) ; self } # [deprecated = "Use `set_reduced_size()` instead."] pub fn reduced_size (& mut self , val : bool) -> & mut Self { self . set_reduced_size (val) ; self } }
};
}
