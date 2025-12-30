// Generated macro for impl_1201 (impl)
macro_rules! Depcrate_features_gen_ConvolverOptionsimpl_1201 {
() => {
// Module: crate::features::gen_ConvolverOptions
// Provides: {"impl_1201"}
// Dependencies: {}
impl ConvolverOptions { # [doc = "Construct a new `ConvolverOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ConvolverOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_channel_count()` instead."] pub fn channel_count (& mut self , val : u32) -> & mut Self { self . set_channel_count (val) ; self } # [cfg (feature = "ChannelCountMode")] # [deprecated = "Use `set_channel_count_mode()` instead."] pub fn channel_count_mode (& mut self , val : ChannelCountMode) -> & mut Self { self . set_channel_count_mode (val) ; self } # [cfg (feature = "ChannelInterpretation")] # [deprecated = "Use `set_channel_interpretation()` instead."] pub fn channel_interpretation (& mut self , val : ChannelInterpretation) -> & mut Self { self . set_channel_interpretation (val) ; self } # [cfg (feature = "AudioBuffer")] # [deprecated = "Use `set_buffer()` instead."] pub fn buffer (& mut self , val : Option < & AudioBuffer >) -> & mut Self { self . set_buffer (val) ; self } # [deprecated = "Use `set_disable_normalization()` instead."] pub fn disable_normalization (& mut self , val : bool) -> & mut Self { self . set_disable_normalization (val) ; self } }
};
}
