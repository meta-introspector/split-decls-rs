// Generated macro for impl_197 (impl)
macro_rules! Depcrate_features_gen_AudioBufferSourceOptionsimpl_197 {
() => {
// Module: crate::features::gen_AudioBufferSourceOptions
// Provides: {"impl_197"}
// Dependencies: {}
impl AudioBufferSourceOptions { # [doc = "Construct a new `AudioBufferSourceOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (feature = "AudioBuffer")] # [deprecated = "Use `set_buffer()` instead."] pub fn buffer (& mut self , val : Option < & AudioBuffer >) -> & mut Self { self . set_buffer (val) ; self } # [deprecated = "Use `set_detune()` instead."] pub fn detune (& mut self , val : f32) -> & mut Self { self . set_detune (val) ; self } # [deprecated = "Use `set_loop()` instead."] pub fn loop_ (& mut self , val : bool) -> & mut Self { self . set_loop (val) ; self } # [deprecated = "Use `set_loop_end()` instead."] pub fn loop_end (& mut self , val : f64) -> & mut Self { self . set_loop_end (val) ; self } # [deprecated = "Use `set_loop_start()` instead."] pub fn loop_start (& mut self , val : f64) -> & mut Self { self . set_loop_start (val) ; self } # [deprecated = "Use `set_playback_rate()` instead."] pub fn playback_rate (& mut self , val : f32) -> & mut Self { self . set_playback_rate (val) ; self } }
};
}
