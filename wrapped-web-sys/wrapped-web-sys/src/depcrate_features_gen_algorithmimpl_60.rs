// Generated macro for impl_60 (impl)
macro_rules! Depcrate_features_gen_Algorithmimpl_60 {
() => {
// Module: crate::features::gen_Algorithm
// Provides: {"impl_60"}
// Dependencies: {}
impl Algorithm { # [doc = "Construct a new `Algorithm`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `Algorithm`*"] pub fn new (name : & str) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } }
};
}
