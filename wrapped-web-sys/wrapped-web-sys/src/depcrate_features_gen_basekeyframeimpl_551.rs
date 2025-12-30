// Generated macro for impl_551 (impl)
macro_rules! Depcrate_features_gen_BaseKeyframeimpl_551 {
() => {
// Module: crate::features::gen_BaseKeyframe
// Provides: {"impl_551"}
// Dependencies: {}
impl BaseKeyframe { # [doc = "Construct a new `BaseKeyframe`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `BaseKeyframe`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (feature = "CompositeOperation")] # [deprecated = "Use `set_composite()` instead."] pub fn composite (& mut self , val : Option < CompositeOperation >) -> & mut Self { self . set_composite (val) ; self } # [deprecated = "Use `set_easing()` instead."] pub fn easing (& mut self , val : & str) -> & mut Self { self . set_easing (val) ; self } # [deprecated = "Use `set_offset()` instead."] pub fn offset (& mut self , val : Option < f64 >) -> & mut Self { self . set_offset (val) ; self } # [deprecated = "Use `set_simulate_compute_values_failure()` instead."] pub fn simulate_compute_values_failure (& mut self , val : bool) -> & mut Self { self . set_simulate_compute_values_failure (val) ; self } }
};
}
