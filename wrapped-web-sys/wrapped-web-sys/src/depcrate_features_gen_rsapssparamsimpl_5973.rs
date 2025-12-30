// Generated macro for impl_5973 (impl)
macro_rules! Depcrate_features_gen_RsaPssParamsimpl_5973 {
() => {
// Module: crate::features::gen_RsaPssParams
// Provides: {"impl_5973"}
// Dependencies: {}
impl RsaPssParams { # [doc = "Construct a new `RsaPssParams`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RsaPssParams`*"] pub fn new (name : & str , salt_length : u32) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret . set_salt_length (salt_length) ; ret } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } # [deprecated = "Use `set_salt_length()` instead."] pub fn salt_length (& mut self , val : u32) -> & mut Self { self . set_salt_length (val) ; self } }
};
}
