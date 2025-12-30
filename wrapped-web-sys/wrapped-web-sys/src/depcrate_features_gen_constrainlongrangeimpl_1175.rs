// Generated macro for impl_1175 (impl)
macro_rules! Depcrate_features_gen_ConstrainLongRangeimpl_1175 {
() => {
// Module: crate::features::gen_ConstrainLongRange
// Provides: {"impl_1175"}
// Dependencies: {}
impl ConstrainLongRange { # [doc = "Construct a new `ConstrainLongRange`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ConstrainLongRange`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_exact()` instead."] pub fn exact (& mut self , val : i32) -> & mut Self { self . set_exact (val) ; self } # [deprecated = "Use `set_ideal()` instead."] pub fn ideal (& mut self , val : i32) -> & mut Self { self . set_ideal (val) ; self } # [deprecated = "Use `set_max()` instead."] pub fn max (& mut self , val : i32) -> & mut Self { self . set_max (val) ; self } # [deprecated = "Use `set_min()` instead."] pub fn min (& mut self , val : i32) -> & mut Self { self . set_min (val) ; self } }
};
}
