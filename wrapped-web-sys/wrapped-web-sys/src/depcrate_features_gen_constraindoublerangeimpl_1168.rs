// Generated macro for impl_1168 (impl)
macro_rules! Depcrate_features_gen_ConstrainDoubleRangeimpl_1168 {
() => {
// Module: crate::features::gen_ConstrainDoubleRange
// Provides: {"impl_1168"}
// Dependencies: {}
impl ConstrainDoubleRange { # [doc = "Construct a new `ConstrainDoubleRange`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_exact()` instead."] pub fn exact (& mut self , val : f64) -> & mut Self { self . set_exact (val) ; self } # [deprecated = "Use `set_ideal()` instead."] pub fn ideal (& mut self , val : f64) -> & mut Self { self . set_ideal (val) ; self } # [deprecated = "Use `set_max()` instead."] pub fn max (& mut self , val : f64) -> & mut Self { self . set_max (val) ; self } # [deprecated = "Use `set_min()` instead."] pub fn min (& mut self , val : f64) -> & mut Self { self . set_min (val) ; self } }
};
}
