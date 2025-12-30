// Generated macro for impl_91 (impl)
macro_rules! Depcrate_features_gen_AnalyserOptionsimpl_91 {
() => {
// Module: crate::features::gen_AnalyserOptions
// Provides: {"impl_91"}
// Dependencies: {}
impl AnalyserOptions { # [doc = "Construct a new `AnalyserOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_channel_count()` instead."] pub fn channel_count (& mut self , val : u32) -> & mut Self { self . set_channel_count (val) ; self } # [cfg (feature = "ChannelCountMode")] # [deprecated = "Use `set_channel_count_mode()` instead."] pub fn channel_count_mode (& mut self , val : ChannelCountMode) -> & mut Self { self . set_channel_count_mode (val) ; self } # [cfg (feature = "ChannelInterpretation")] # [deprecated = "Use `set_channel_interpretation()` instead."] pub fn channel_interpretation (& mut self , val : ChannelInterpretation) -> & mut Self { self . set_channel_interpretation (val) ; self } # [deprecated = "Use `set_fft_size()` instead."] pub fn fft_size (& mut self , val : u32) -> & mut Self { self . set_fft_size (val) ; self } # [deprecated = "Use `set_max_decibels()` instead."] pub fn max_decibels (& mut self , val : f64) -> & mut Self { self . set_max_decibels (val) ; self } # [deprecated = "Use `set_min_decibels()` instead."] pub fn min_decibels (& mut self , val : f64) -> & mut Self { self . set_min_decibels (val) ; self } # [deprecated = "Use `set_smoothing_time_constant()` instead."] pub fn smoothing_time_constant (& mut self , val : f64) -> & mut Self { self . set_smoothing_time_constant (val) ; self } }
};
}
