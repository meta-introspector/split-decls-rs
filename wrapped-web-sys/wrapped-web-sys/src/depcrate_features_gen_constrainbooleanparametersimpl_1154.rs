// Generated macro for impl_1154 (impl)
macro_rules! Depcrate_features_gen_ConstrainBooleanParametersimpl_1154 {
() => {
// Module: crate::features::gen_ConstrainBooleanParameters
// Provides: {"impl_1154"}
// Dependencies: {}
impl ConstrainBooleanParameters { # [doc = "Construct a new `ConstrainBooleanParameters`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `ConstrainBooleanParameters`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_exact()` instead."] pub fn exact (& mut self , val : bool) -> & mut Self { self . set_exact (val) ; self } # [deprecated = "Use `set_ideal()` instead."] pub fn ideal (& mut self , val : bool) -> & mut Self { self . set_ideal (val) ; self } }
};
}
