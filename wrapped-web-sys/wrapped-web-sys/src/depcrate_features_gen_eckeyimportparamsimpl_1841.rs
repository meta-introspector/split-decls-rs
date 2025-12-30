// Generated macro for impl_1841 (impl)
macro_rules! Depcrate_features_gen_EcKeyImportParamsimpl_1841 {
() => {
// Module: crate::features::gen_EcKeyImportParams
// Provides: {"impl_1841"}
// Dependencies: {}
impl EcKeyImportParams { # [doc = "Construct a new `EcKeyImportParams`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `EcKeyImportParams`*"] pub fn new (name : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } # [deprecated = "Use `set_named_curve()` instead."] pub fn named_curve (& mut self , val : & str) -> & mut Self { self . set_named_curve (val) ; self } }
};
}
