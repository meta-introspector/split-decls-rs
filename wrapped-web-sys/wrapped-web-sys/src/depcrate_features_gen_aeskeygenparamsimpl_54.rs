// Generated macro for impl_54 (impl)
macro_rules! Depcrate_features_gen_AesKeyGenParamsimpl_54 {
() => {
// Module: crate::features::gen_AesKeyGenParams
// Provides: {"impl_54"}
// Dependencies: {}
impl AesKeyGenParams { # [doc = "Construct a new `AesKeyGenParams`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AesKeyGenParams`*"] pub fn new (name : & str , length : u16) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret . set_length (length) ; ret } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } # [deprecated = "Use `set_length()` instead."] pub fn length (& mut self , val : u16) -> & mut Self { self . set_length (val) ; self } }
};
}
