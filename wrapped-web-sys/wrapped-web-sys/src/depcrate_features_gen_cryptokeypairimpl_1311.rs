// Generated macro for impl_1311 (impl)
macro_rules! Depcrate_features_gen_CryptoKeyPairimpl_1311 {
() => {
// Module: crate::features::gen_CryptoKeyPair
// Provides: {"impl_1311"}
// Dependencies: {}
impl CryptoKeyPair { # [cfg (feature = "CryptoKey")] # [doc = "Construct a new `CryptoKeyPair`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `CryptoKey`, `CryptoKeyPair`*"] pub fn new (private_key : & CryptoKey , public_key : & CryptoKey) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_private_key (private_key) ; ret . set_public_key (public_key) ; ret } # [cfg (feature = "CryptoKey")] # [deprecated = "Use `set_private_key()` instead."] pub fn private_key (& mut self , val : & CryptoKey) -> & mut Self { self . set_private_key (val) ; self } # [cfg (feature = "CryptoKey")] # [deprecated = "Use `set_public_key()` instead."] pub fn public_key (& mut self , val : & CryptoKey) -> & mut Self { self . set_public_key (val) ; self } }
};
}
