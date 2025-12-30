// Generated macro for impl_5722 (impl)
macro_rules! Depcrate_features_gen_QueuingStrategyInitimpl_5722 {
() => {
// Module: crate::features::gen_QueuingStrategyInit
// Provides: {"impl_5722"}
// Dependencies: {}
impl QueuingStrategyInit { # [doc = "Construct a new `QueuingStrategyInit`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `QueuingStrategyInit`*"] pub fn new (high_water_mark : f64) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_high_water_mark (high_water_mark) ; ret } # [deprecated = "Use `set_high_water_mark()` instead."] pub fn high_water_mark (& mut self , val : f64) -> & mut Self { self . set_high_water_mark (val) ; self } }
};
}
