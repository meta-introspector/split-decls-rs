// Generated macro for impl_1847 (impl)
macro_rules! Depcrate_features_gen_EcdhKeyDeriveParamsimpl_1847 {
() => {
// Module: crate::features::gen_EcdhKeyDeriveParams
// Provides: {"impl_1847"}
// Dependencies: {}
impl EcdhKeyDeriveParams { # [cfg (feature = "CryptoKey")] # [doc = "Construct a new `EcdhKeyDeriveParams`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `CryptoKey`, `EcdhKeyDeriveParams`*"] pub fn new (name : & str , public : & CryptoKey) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret . set_public (public) ; ret } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } # [cfg (feature = "CryptoKey")] # [deprecated = "Use `set_public()` instead."] pub fn public (& mut self , val : & CryptoKey) -> & mut Self { self . set_public (val) ; self } }
};
}
