// Generated macro for impl_4483 (impl)
macro_rules! Depcrate_features_gen_MediaRecorderOptionsimpl_4483 {
() => {
// Module: crate::features::gen_MediaRecorderOptions
// Provides: {"impl_4483"}
// Dependencies: {}
impl MediaRecorderOptions { # [doc = "Construct a new `MediaRecorderOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_audio_bits_per_second()` instead."] pub fn audio_bits_per_second (& mut self , val : u32) -> & mut Self { self . set_audio_bits_per_second (val) ; self } # [deprecated = "Use `set_bits_per_second()` instead."] pub fn bits_per_second (& mut self , val : u32) -> & mut Self { self . set_bits_per_second (val) ; self } # [deprecated = "Use `set_mime_type()` instead."] pub fn mime_type (& mut self , val : & str) -> & mut Self { self . set_mime_type (val) ; self } # [deprecated = "Use `set_video_bits_per_second()` instead."] pub fn video_bits_per_second (& mut self , val : u32) -> & mut Self { self . set_video_bits_per_second (val) ; self } }
};
}
