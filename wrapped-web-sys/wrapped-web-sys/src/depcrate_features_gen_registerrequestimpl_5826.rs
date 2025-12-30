// Generated macro for impl_5826 (impl)
macro_rules! Depcrate_features_gen_RegisterRequestimpl_5826 {
() => {
// Module: crate::features::gen_RegisterRequest
// Provides: {"impl_5826"}
// Dependencies: {}
impl RegisterRequest { # [doc = "Construct a new `RegisterRequest`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `RegisterRequest`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_challenge()` instead."] pub fn challenge (& mut self , val : & str) -> & mut Self { self . set_challenge (val) ; self } # [deprecated = "Use `set_version()` instead."] pub fn version (& mut self , val : & str) -> & mut Self { self . set_version (val) ; self } }
};
}
