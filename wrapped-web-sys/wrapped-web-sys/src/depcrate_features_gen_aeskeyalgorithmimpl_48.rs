// Generated macro for impl_48 (impl)
macro_rules! Depcrate_features_gen_AesKeyAlgorithmimpl_48 {
() => {
// Module: crate::features::gen_AesKeyAlgorithm
// Provides: {"impl_48"}
// Dependencies: {}
impl AesKeyAlgorithm { # [doc = "Construct a new `AesKeyAlgorithm`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AesKeyAlgorithm`*"] pub fn new (name : & str , length : u16) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret . set_length (length) ; ret } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } # [deprecated = "Use `set_length()` instead."] pub fn length (& mut self , val : u16) -> & mut Self { self . set_length (val) ; self } }
};
}
