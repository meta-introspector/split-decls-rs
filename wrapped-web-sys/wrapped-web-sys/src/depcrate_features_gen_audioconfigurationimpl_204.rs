// Generated macro for impl_204 (impl)
macro_rules! Depcrate_features_gen_AudioConfigurationimpl_204 {
() => {
// Module: crate::features::gen_AudioConfiguration
// Provides: {"impl_204"}
// Dependencies: {}
impl AudioConfiguration { # [doc = "Construct a new `AudioConfiguration`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_bitrate()` instead."] pub fn bitrate (& mut self , val : f64) -> & mut Self { self . set_bitrate (val) ; self } # [deprecated = "Use `set_channels()` instead."] pub fn channels (& mut self , val : & str) -> & mut Self { self . set_channels (val) ; self } # [deprecated = "Use `set_content_type()` instead."] pub fn content_type (& mut self , val : & str) -> & mut Self { self . set_content_type (val) ; self } # [deprecated = "Use `set_samplerate()` instead."] pub fn samplerate (& mut self , val : u32) -> & mut Self { self . set_samplerate (val) ; self } }
};
}
