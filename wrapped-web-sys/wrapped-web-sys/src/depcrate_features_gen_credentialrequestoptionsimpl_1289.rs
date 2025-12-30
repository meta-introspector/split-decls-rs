// Generated macro for impl_1289 (impl)
macro_rules! Depcrate_features_gen_CredentialRequestOptionsimpl_1289 {
() => {
// Module: crate::features::gen_CredentialRequestOptions
// Provides: {"impl_1289"}
// Dependencies: {}
impl CredentialRequestOptions { # [doc = "Construct a new `CredentialRequestOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `CredentialRequestOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (feature = "PublicKeyCredentialRequestOptions")] # [deprecated = "Use `set_public_key()` instead."] pub fn public_key (& mut self , val : & PublicKeyCredentialRequestOptions) -> & mut Self { self . set_public_key (val) ; self } # [cfg (feature = "AbortSignal")] # [deprecated = "Use `set_signal()` instead."] pub fn signal (& mut self , val : & AbortSignal) -> & mut Self { self . set_signal (val) ; self } }
};
}
