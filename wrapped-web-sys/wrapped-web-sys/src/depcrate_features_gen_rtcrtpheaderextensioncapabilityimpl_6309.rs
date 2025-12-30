// Generated macro for impl_6309 (impl)
macro_rules! Depcrate_features_gen_RtcRtpHeaderExtensionCapabilityimpl_6309 {
() => {
// Module: crate::features::gen_RtcRtpHeaderExtensionCapability
// Provides: {"impl_6309"}
// Dependencies: {}
impl RtcRtpHeaderExtensionCapability { # [doc = "Construct a new `RtcRtpHeaderExtensionCapability`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionCapability`*"] pub fn new (uri : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_uri (uri) ; ret } # [deprecated = "Use `set_uri()` instead."] pub fn uri (& mut self , val : & str) -> & mut Self { self . set_uri (val) ; self } }
};
}
