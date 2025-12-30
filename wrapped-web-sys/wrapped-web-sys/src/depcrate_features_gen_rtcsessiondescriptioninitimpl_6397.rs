// Generated macro for impl_6397 (impl)
macro_rules! Depcrate_features_gen_RtcSessionDescriptionInitimpl_6397 {
() => {
// Module: crate::features::gen_RtcSessionDescriptionInit
// Provides: {"impl_6397"}
// Dependencies: {}
impl RtcSessionDescriptionInit { # [cfg (feature = "RtcSdpType")] # [doc = "Construct a new `RtcSessionDescriptionInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcSdpType`, `RtcSessionDescriptionInit`*"] pub fn new (type_ : RtcSdpType) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_type (type_) ; ret } # [deprecated = "Use `set_sdp()` instead."] pub fn sdp (& mut self , val : & str) -> & mut Self { self . set_sdp (val) ; self } # [cfg (feature = "RtcSdpType")] # [deprecated = "Use `set_type()` instead."] pub fn type_ (& mut self , val : RtcSdpType) -> & mut Self { self . set_type (val) ; self } }
};
}
