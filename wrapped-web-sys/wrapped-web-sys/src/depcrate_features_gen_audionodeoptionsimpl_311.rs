// Generated macro for impl_311 (impl)
macro_rules! Depcrate_features_gen_AudioNodeOptionsimpl_311 {
() => {
// Module: crate::features::gen_AudioNodeOptions
// Provides: {"impl_311"}
// Dependencies: {}
impl AudioNodeOptions { # [doc = "Construct a new `AudioNodeOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AudioNodeOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_channel_count()` instead."] pub fn channel_count (& mut self , val : u32) -> & mut Self { self . set_channel_count (val) ; self } # [cfg (feature = "ChannelCountMode")] # [deprecated = "Use `set_channel_count_mode()` instead."] pub fn channel_count_mode (& mut self , val : ChannelCountMode) -> & mut Self { self . set_channel_count_mode (val) ; self } # [cfg (feature = "ChannelInterpretation")] # [deprecated = "Use `set_channel_interpretation()` instead."] pub fn channel_interpretation (& mut self , val : ChannelInterpretation) -> & mut Self { self . set_channel_interpretation (val) ; self } }
};
}
