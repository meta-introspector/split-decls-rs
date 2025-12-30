// Generated macro for impl_6921 (impl)
macro_rules! Depcrate_features_gen_StereoPannerOptionsimpl_6921 {
() => {
// Module: crate::features::gen_StereoPannerOptions
// Provides: {"impl_6921"}
// Dependencies: {}
impl StereoPannerOptions { # [doc = "Construct a new `StereoPannerOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `StereoPannerOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_channel_count()` instead."] pub fn channel_count (& mut self , val : u32) -> & mut Self { self . set_channel_count (val) ; self } # [cfg (feature = "ChannelCountMode")] # [deprecated = "Use `set_channel_count_mode()` instead."] pub fn channel_count_mode (& mut self , val : ChannelCountMode) -> & mut Self { self . set_channel_count_mode (val) ; self } # [cfg (feature = "ChannelInterpretation")] # [deprecated = "Use `set_channel_interpretation()` instead."] pub fn channel_interpretation (& mut self , val : ChannelInterpretation) -> & mut Self { self . set_channel_interpretation (val) ; self } # [deprecated = "Use `set_pan()` instead."] pub fn pan (& mut self , val : f32) -> & mut Self { self . set_pan (val) ; self } }
};
}
