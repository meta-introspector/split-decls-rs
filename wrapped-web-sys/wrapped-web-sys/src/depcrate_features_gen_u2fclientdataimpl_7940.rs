// Generated macro for impl_7940 (impl)
macro_rules! Depcrate_features_gen_U2fClientDataimpl_7940 {
() => {
// Module: crate::features::gen_U2fClientData
// Provides: {"impl_7940"}
// Dependencies: {}
impl U2fClientData { # [doc = "Construct a new `U2fClientData`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `U2fClientData`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_challenge()` instead."] pub fn challenge (& mut self , val : & str) -> & mut Self { self . set_challenge (val) ; self } # [deprecated = "Use `set_origin()` instead."] pub fn origin (& mut self , val : & str) -> & mut Self { self . set_origin (val) ; self } # [deprecated = "Use `set_typ()` instead."] pub fn typ (& mut self , val : & str) -> & mut Self { self . set_typ (val) ; self } }
};
}
