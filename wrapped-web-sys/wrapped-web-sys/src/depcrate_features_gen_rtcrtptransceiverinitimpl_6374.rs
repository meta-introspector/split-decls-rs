// Generated macro for impl_6374 (impl)
macro_rules! Depcrate_features_gen_RtcRtpTransceiverInitimpl_6374 {
() => {
// Module: crate::features::gen_RtcRtpTransceiverInit
// Provides: {"impl_6374"}
// Dependencies: {}
impl RtcRtpTransceiverInit { # [doc = "Construct a new `RtcRtpTransceiverInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverInit`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (feature = "RtcRtpTransceiverDirection")] # [deprecated = "Use `set_direction()` instead."] pub fn direction (& mut self , val : RtcRtpTransceiverDirection) -> & mut Self { self . set_direction (val) ; self } # [deprecated = "Use `set_send_encodings()` instead."] pub fn send_encodings (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_send_encodings (val) ; self } # [deprecated = "Use `set_streams()` instead."] pub fn streams (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_streams (val) ; self } }
};
}
