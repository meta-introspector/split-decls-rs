// Generated macro for impl_8447 (impl)
macro_rules! Depcrate_features_gen_WaveShaperOptionsimpl_8447 {
() => {
// Module: crate::features::gen_WaveShaperOptions
// Provides: {"impl_8447"}
// Dependencies: {}
impl WaveShaperOptions { # [doc = "Construct a new `WaveShaperOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `WaveShaperOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_channel_count()` instead."] pub fn channel_count (& mut self , val : u32) -> & mut Self { self . set_channel_count (val) ; self } # [cfg (feature = "ChannelCountMode")] # [deprecated = "Use `set_channel_count_mode()` instead."] pub fn channel_count_mode (& mut self , val : ChannelCountMode) -> & mut Self { self . set_channel_count_mode (val) ; self } # [cfg (feature = "ChannelInterpretation")] # [deprecated = "Use `set_channel_interpretation()` instead."] pub fn channel_interpretation (& mut self , val : ChannelInterpretation) -> & mut Self { self . set_channel_interpretation (val) ; self } # [deprecated = "Use `set_curve()` instead."] pub fn curve (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_curve (val) ; self } # [cfg (feature = "OverSampleType")] # [deprecated = "Use `set_oversample()` instead."] pub fn oversample (& mut self , val : OverSampleType) -> & mut Self { self . set_oversample (val) ; self } }
};
}
