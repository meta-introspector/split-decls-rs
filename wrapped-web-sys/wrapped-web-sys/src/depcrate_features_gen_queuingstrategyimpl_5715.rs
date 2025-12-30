// Generated macro for impl_5715 (impl)
macro_rules! Depcrate_features_gen_QueuingStrategyimpl_5715 {
() => {
// Module: crate::features::gen_QueuingStrategy
// Provides: {"impl_5715"}
// Dependencies: {}
impl QueuingStrategy { # [doc = "Construct a new `QueuingStrategy`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `QueuingStrategy`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_high_water_mark()` instead."] pub fn high_water_mark (& mut self , val : f64) -> & mut Self { self . set_high_water_mark (val) ; self } # [deprecated = "Use `set_size()` instead."] pub fn size (& mut self , val : & :: js_sys :: Function) -> & mut Self { self . set_size (val) ; self } }
};
}
