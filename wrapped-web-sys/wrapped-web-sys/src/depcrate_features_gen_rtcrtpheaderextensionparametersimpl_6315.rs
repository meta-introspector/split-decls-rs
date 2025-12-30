// Generated macro for impl_6315 (impl)
macro_rules! Depcrate_features_gen_RtcRtpHeaderExtensionParametersimpl_6315 {
() => {
// Module: crate::features::gen_RtcRtpHeaderExtensionParameters
// Provides: {"impl_6315"}
// Dependencies: {}
impl RtcRtpHeaderExtensionParameters { # [doc = "Construct a new `RtcRtpHeaderExtensionParameters`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_encrypted()` instead."] pub fn encrypted (& mut self , val : bool) -> & mut Self { self . set_encrypted (val) ; self } # [deprecated = "Use `set_id()` instead."] pub fn id (& mut self , val : u16) -> & mut Self { self . set_id (val) ; self } # [deprecated = "Use `set_uri()` instead."] pub fn uri (& mut self , val : & str) -> & mut Self { self . set_uri (val) ; self } }
};
}
