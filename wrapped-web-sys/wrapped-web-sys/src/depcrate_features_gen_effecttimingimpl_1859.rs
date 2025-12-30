// Generated macro for impl_1859 (impl)
macro_rules! Depcrate_features_gen_EffectTimingimpl_1859 {
() => {
// Module: crate::features::gen_EffectTiming
// Provides: {"impl_1859"}
// Dependencies: {}
impl EffectTiming { # [doc = "Construct a new `EffectTiming`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `EffectTiming`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_delay()` instead."] pub fn delay (& mut self , val : f64) -> & mut Self { self . set_delay (val) ; self } # [cfg (feature = "PlaybackDirection")] # [deprecated = "Use `set_direction()` instead."] pub fn direction (& mut self , val : PlaybackDirection) -> & mut Self { self . set_direction (val) ; self } # [deprecated = "Use `set_duration()` instead."] pub fn duration (& mut self , val : & :: wasm_bindgen :: JsValue) -> & mut Self { self . set_duration (val) ; self } # [deprecated = "Use `set_easing()` instead."] pub fn easing (& mut self , val : & str) -> & mut Self { self . set_easing (val) ; self } # [deprecated = "Use `set_end_delay()` instead."] pub fn end_delay (& mut self , val : f64) -> & mut Self { self . set_end_delay (val) ; self } # [cfg (feature = "FillMode")] # [deprecated = "Use `set_fill()` instead."] pub fn fill (& mut self , val : FillMode) -> & mut Self { self . set_fill (val) ; self } # [deprecated = "Use `set_iteration_start()` instead."] pub fn iteration_start (& mut self , val : f64) -> & mut Self { self . set_iteration_start (val) ; self } # [deprecated = "Use `set_iterations()` instead."] pub fn iterations (& mut self , val : f64) -> & mut Self { self . set_iterations (val) ; self } }
};
}
