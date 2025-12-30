// Generated macro for impl_5020 (impl)
macro_rules! Depcrate_features_gen_OfflineAudioContextOptionsimpl_5020 {
() => {
// Module: crate::features::gen_OfflineAudioContextOptions
// Provides: {"impl_5020"}
// Dependencies: {}
impl OfflineAudioContextOptions { # [doc = "Construct a new `OfflineAudioContextOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `OfflineAudioContextOptions`*"] pub fn new (length : u32 , sample_rate : f32) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_length (length) ; ret . set_sample_rate (sample_rate) ; ret } # [deprecated = "Use `set_length()` instead."] pub fn length (& mut self , val : u32) -> & mut Self { self . set_length (val) ; self } # [deprecated = "Use `set_number_of_channels()` instead."] pub fn number_of_channels (& mut self , val : u32) -> & mut Self { self . set_number_of_channels (val) ; self } # [deprecated = "Use `set_sample_rate()` instead."] pub fn sample_rate (& mut self , val : f32) -> & mut Self { self . set_sample_rate (val) ; self } }
};
}
