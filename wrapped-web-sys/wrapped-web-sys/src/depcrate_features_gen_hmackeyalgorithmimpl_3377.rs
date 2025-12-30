// Generated macro for impl_3377 (impl)
macro_rules! Depcrate_features_gen_HmacKeyAlgorithmimpl_3377 {
() => {
// Module: crate::features::gen_HmacKeyAlgorithm
// Provides: {"impl_3377"}
// Dependencies: {}
impl HmacKeyAlgorithm { # [cfg (feature = "KeyAlgorithm")] # [doc = "Construct a new `HmacKeyAlgorithm`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`, `KeyAlgorithm`*"] pub fn new (name : & str , hash : & KeyAlgorithm , length : u32) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret . set_hash (hash) ; ret . set_length (length) ; ret } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } # [cfg (feature = "KeyAlgorithm")] # [deprecated = "Use `set_hash()` instead."] pub fn hash (& mut self , val : & KeyAlgorithm) -> & mut Self { self . set_hash (val) ; self } # [deprecated = "Use `set_length()` instead."] pub fn length (& mut self , val : u32) -> & mut Self { self . set_length (val) ; self } }
};
}
