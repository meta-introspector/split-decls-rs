// Generated macro for impl_6349 (impl)
macro_rules! Depcrate_features_gen_RtcRtpSourceEntryimpl_6349 {
() => {
// Module: crate::features::gen_RtcRtpSourceEntry
// Provides: {"impl_6349"}
// Dependencies: {}
impl RtcRtpSourceEntry { # [cfg (feature = "RtcRtpSourceEntryType")] # [doc = "Construct a new `RtcRtpSourceEntry`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`, `RtcRtpSourceEntryType`*"] pub fn new (source : u32 , timestamp : f64 , source_type : RtcRtpSourceEntryType) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_source (source) ; ret . set_timestamp (timestamp) ; ret . set_source_type (source_type) ; ret } # [deprecated = "Use `set_audio_level()` instead."] pub fn audio_level (& mut self , val : f64) -> & mut Self { self . set_audio_level (val) ; self } # [deprecated = "Use `set_source()` instead."] pub fn source (& mut self , val : u32) -> & mut Self { self . set_source (val) ; self } # [deprecated = "Use `set_timestamp()` instead."] pub fn timestamp (& mut self , val : f64) -> & mut Self { self . set_timestamp (val) ; self } # [deprecated = "Use `set_voice_activity_flag()` instead."] pub fn voice_activity_flag (& mut self , val : Option < bool >) -> & mut Self { self . set_voice_activity_flag (val) ; self } # [cfg (feature = "RtcRtpSourceEntryType")] # [deprecated = "Use `set_source_type()` instead."] pub fn source_type (& mut self , val : RtcRtpSourceEntryType) -> & mut Self { self . set_source_type (val) ; self } }
};
}
