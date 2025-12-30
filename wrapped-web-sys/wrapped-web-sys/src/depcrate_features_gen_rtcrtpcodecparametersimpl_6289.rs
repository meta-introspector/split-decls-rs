// Generated macro for impl_6289 (impl)
macro_rules! Depcrate_features_gen_RtcRtpCodecParametersimpl_6289 {
() => {
// Module: crate::features::gen_RtcRtpCodecParameters
// Provides: {"impl_6289"}
// Dependencies: {}
impl RtcRtpCodecParameters { # [doc = "Construct a new `RtcRtpCodecParameters`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_channels()` instead."] pub fn channels (& mut self , val : u16) -> & mut Self { self . set_channels (val) ; self } # [deprecated = "Use `set_clock_rate()` instead."] pub fn clock_rate (& mut self , val : u32) -> & mut Self { self . set_clock_rate (val) ; self } # [deprecated = "Use `set_mime_type()` instead."] pub fn mime_type (& mut self , val : & str) -> & mut Self { self . set_mime_type (val) ; self } # [deprecated = "Use `set_payload_type()` instead."] pub fn payload_type (& mut self , val : u16) -> & mut Self { self . set_payload_type (val) ; self } # [deprecated = "Use `set_sdp_fmtp_line()` instead."] pub fn sdp_fmtp_line (& mut self , val : & str) -> & mut Self { self . set_sdp_fmtp_line (val) ; self } }
};
}
