// Generated macro for impl_1829 (impl)
macro_rules! Depcrate_features_gen_EcKeyAlgorithmimpl_1829 {
() => {
// Module: crate::features::gen_EcKeyAlgorithm
// Provides: {"impl_1829"}
// Dependencies: {}
impl EcKeyAlgorithm { # [doc = "Construct a new `EcKeyAlgorithm`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `EcKeyAlgorithm`*"] pub fn new (name : & str , named_curve : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret . set_named_curve (named_curve) ; ret } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } # [deprecated = "Use `set_named_curve()` instead."] pub fn named_curve (& mut self , val : & str) -> & mut Self { self . set_named_curve (val) ; self } }
};
}
