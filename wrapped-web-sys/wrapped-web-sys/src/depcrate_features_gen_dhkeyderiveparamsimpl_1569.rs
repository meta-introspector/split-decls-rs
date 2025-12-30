// Generated macro for impl_1569 (impl)
macro_rules! Depcrate_features_gen_DhKeyDeriveParamsimpl_1569 {
() => {
// Module: crate::features::gen_DhKeyDeriveParams
// Provides: {"impl_1569"}
// Dependencies: {}
impl DhKeyDeriveParams { # [cfg (feature = "CryptoKey")] # [doc = "Construct a new `DhKeyDeriveParams`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `CryptoKey`, `DhKeyDeriveParams`*"] pub fn new (name : & str , public : & CryptoKey) -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret . set_name (name) ; ret . set_public (public) ; ret } # [deprecated = "Use `set_name()` instead."] pub fn name (& mut self , val : & str) -> & mut Self { self . set_name (val) ; self } # [cfg (feature = "CryptoKey")] # [deprecated = "Use `set_public()` instead."] pub fn public (& mut self , val : & CryptoKey) -> & mut Self { self . set_public (val) ; self } }
};
}
