// Generated macro for impl_4114 (impl)
macro_rules! Depcrate_features_gen_KeyAlgorithmimpl_4114 {
() => {
// Module: crate::features::gen_KeyAlgorithm
// Provides: {"impl_4114"}
// Dependencies: {}
impl KeyAlgorithm { # [doc = "Construct a new `KeyAlgorithm`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `KeyAlgorithm`*"] pub fn new (name : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } }
};
}
