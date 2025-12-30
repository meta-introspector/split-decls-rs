// Generated macro for impl_2426 (impl)
macro_rules! Depcrate_features_gen_GainOptionsimpl_2426 {
() => {
// Module: crate::features::gen_GainOptions
// Provides: {"impl_2426"}
// Dependencies: {}
impl GainOptions { # [doc = "Construct a new `GainOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `GainOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_channel_count()` instead."] pub fn channel_count (& mut self , val : u32) -> & mut Self { self . set_channel_count (val) ; self } # [cfg (feature = "ChannelCountMode")] # [deprecated = "Use `set_channel_count_mode()` instead."] pub fn channel_count_mode (& mut self , val : ChannelCountMode) -> & mut Self { self . set_channel_count_mode (val) ; self } # [cfg (feature = "ChannelInterpretation")] # [deprecated = "Use `set_channel_interpretation()` instead."] pub fn channel_interpretation (& mut self , val : ChannelInterpretation) -> & mut Self { self . set_channel_interpretation (val) ; self } # [deprecated = "Use `set_gain()` instead."] pub fn gain (& mut self , val : f32) -> & mut Self { self . set_gain (val) ; self } }
};
}
