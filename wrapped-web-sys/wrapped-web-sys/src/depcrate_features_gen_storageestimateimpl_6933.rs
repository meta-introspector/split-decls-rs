// Generated macro for impl_6933 (impl)
macro_rules! Depcrate_features_gen_StorageEstimateimpl_6933 {
() => {
// Module: crate::features::gen_StorageEstimate
// Provides: {"impl_6933"}
// Dependencies: {}
impl StorageEstimate { # [doc = "Construct a new `StorageEstimate`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `StorageEstimate`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_quota()` instead."] pub fn quota (& mut self , val : f64) -> & mut Self { self . set_quota (val) ; self } # [deprecated = "Use `set_usage()` instead."] pub fn usage (& mut self , val : f64) -> & mut Self { self . set_usage (val) ; self } }
};
}
