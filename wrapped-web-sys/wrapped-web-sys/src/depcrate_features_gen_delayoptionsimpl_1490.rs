// Generated macro for impl_1490 (impl)
macro_rules! Depcrate_features_gen_DelayOptionsimpl_1490 {
() => {
// Module: crate::features::gen_DelayOptions
// Provides: {"impl_1490"}
// Dependencies: {}
impl DelayOptions { # [doc = "Construct a new `DelayOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `DelayOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_channel_count()` instead."] pub fn channel_count (& mut self , val : u32) -> & mut Self { self . set_channel_count (val) ; self } # [cfg (feature = "ChannelCountMode")] # [deprecated = "Use `set_channel_count_mode()` instead."] pub fn channel_count_mode (& mut self , val : ChannelCountMode) -> & mut Self { self . set_channel_count_mode (val) ; self } # [cfg (feature = "ChannelInterpretation")] # [deprecated = "Use `set_channel_interpretation()` instead."] pub fn channel_interpretation (& mut self , val : ChannelInterpretation) -> & mut Self { self . set_channel_interpretation (val) ; self } # [deprecated = "Use `set_delay_time()` instead."] pub fn delay_time (& mut self , val : f64) -> & mut Self { self . set_delay_time (val) ; self } # [deprecated = "Use `set_max_delay_time()` instead."] pub fn max_delay_time (& mut self , val : f64) -> & mut Self { self . set_max_delay_time (val) ; self } }
};
}
