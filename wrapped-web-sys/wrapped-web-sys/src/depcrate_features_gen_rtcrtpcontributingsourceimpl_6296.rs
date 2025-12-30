// Generated macro for impl_6296 (impl)
macro_rules! Depcrate_features_gen_RtcRtpContributingSourceimpl_6296 {
() => {
// Module: crate::features::gen_RtcRtpContributingSource
// Provides: {"impl_6296"}
// Dependencies: {}
impl RtcRtpContributingSource { # [doc = "Construct a new `RtcRtpContributingSource`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RtcRtpContributingSource`*"] pub fn new (source : u32 , timestamp : f64) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_source (source) ; ret . set_timestamp (timestamp) ; ret } # [deprecated = "Use `set_audio_level()` instead."] pub fn audio_level (& mut self , val : f64) -> & mut Self { self . set_audio_level (val) ; self } # [deprecated = "Use `set_source()` instead."] pub fn source (& mut self , val : u32) -> & mut Self { self . set_source (val) ; self } # [deprecated = "Use `set_timestamp()` instead."] pub fn timestamp (& mut self , val : f64) -> & mut Self { self . set_timestamp (val) ; self } }
};
}
