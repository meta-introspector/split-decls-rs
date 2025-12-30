// Generated macro for impl_6009 (impl)
macro_rules! Depcrate_features_gen_RtcConfigurationimpl_6009 {
() => {
// Module: crate::features::gen_RtcConfiguration
// Provides: {"impl_6009"}
// Dependencies: {}
impl RtcConfiguration { # [doc = "Construct a new `RtcConfiguration`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (feature = "RtcBundlePolicy")] # [deprecated = "Use `set_bundle_policy()` instead."] pub fn bundle_policy (& mut self , val : RtcBundlePolicy) -> & mut Self { self . set_bundle_policy (val) ; self } # [deprecated = "Use `set_certificates()` instead."] pub fn certificates (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_certificates (val) ; self } # [deprecated = "Use `set_ice_servers()` instead."] pub fn ice_servers (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_ice_servers (val) ; self } # [cfg (feature = "RtcIceTransportPolicy")] # [deprecated = "Use `set_ice_transport_policy()` instead."] pub fn ice_transport_policy (& mut self , val : RtcIceTransportPolicy) -> & mut Self { self . set_ice_transport_policy (val) ; self } # [deprecated = "Use `set_peer_identity()` instead."] pub fn peer_identity (& mut self , val : Option < & str >) -> & mut Self { self . set_peer_identity (val) ; self } }
};
}
