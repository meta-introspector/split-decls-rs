// Generated macro for impl_3942 (impl)
macro_rules! Depcrate_features_gen_IirFilterOptionsimpl_3942 {
() => {
// Module: crate::features::gen_IirFilterOptions
// Provides: {"impl_3942"}
// Dependencies: {}
impl IirFilterOptions { # [doc = "Construct a new `IirFilterOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `IirFilterOptions`*"] pub fn new (feedback : & :: wasm_bindgen :: JsValue , feedforward : & :: wasm_bindgen :: JsValue) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_feedback (feedback) ; ret . set_feedforward (feedforward) ; ret } # [deprecated = "Use `set_channel_count()` instead."] pub fn channel_count (& mut self , val : u32) -> & mut Self { self . set_channel_count (val) ; self } # [cfg (feature = "ChannelCountMode")] # [deprecated = "Use `set_channel_count_mode()` instead."] pub fn channel_count_mode (& mut self , val : ChannelCountMode) -> & mut Self { self . set_channel_count_mode (val) ; self } # [cfg (feature = "ChannelInterpretation")] # [deprecated = "Use `set_channel_interpretation()` instead."] pub fn channel_interpretation (& mut self , val : ChannelInterpretation) -> & mut Self { self . set_channel_interpretation (val) ; self } # [deprecated = "Use `set_feedback()` instead."] pub fn feedback (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_feedback (val) ; self } # [deprecated = "Use `set_feedforward()` instead."] pub fn feedforward (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_feedforward (val) ; self } }
};
}
