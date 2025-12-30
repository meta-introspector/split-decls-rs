// Generated macro for impl_6283 (impl)
macro_rules! Depcrate_features_gen_RtcRtpCodecCapabilityimpl_6283 {
() => {
// Module: crate::features::gen_RtcRtpCodecCapability
// Provides: {"impl_6283"}
// Dependencies: {}
impl RtcRtpCodecCapability { # [doc = "Construct a new `RtcRtpCodecCapability`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"] pub fn new (clock_rate : u32 , mime_type : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_clock_rate (clock_rate) ; ret . set_mime_type (mime_type) ; ret } # [deprecated = "Use `set_channels()` instead."] pub fn channels (& mut self , val : u16) -> & mut Self { self . set_channels (val) ; self } # [deprecated = "Use `set_clock_rate()` instead."] pub fn clock_rate (& mut self , val : u32) -> & mut Self { self . set_clock_rate (val) ; self } # [deprecated = "Use `set_mime_type()` instead."] pub fn mime_type (& mut self , val : & str) -> & mut Self { self . set_mime_type (val) ; self } # [deprecated = "Use `set_sdp_fmtp_line()` instead."] pub fn sdp_fmtp_line (& mut self , val : & str) -> & mut Self { self . set_sdp_fmtp_line (val) ; self } }
};
}
