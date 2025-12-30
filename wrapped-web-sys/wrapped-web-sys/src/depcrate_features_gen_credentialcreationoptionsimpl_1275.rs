// Generated macro for impl_1275 (impl)
macro_rules! Depcrate_features_gen_CredentialCreationOptionsimpl_1275 {
() => {
// Module: crate::features::gen_CredentialCreationOptions
// Provides: {"impl_1275"}
// Dependencies: {}
impl CredentialCreationOptions { # [doc = "Construct a new `CredentialCreationOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `CredentialCreationOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (feature = "PublicKeyCredentialCreationOptions")] # [deprecated = "Use `set_public_key()` instead."] pub fn public_key (& mut self , val : & PublicKeyCredentialCreationOptions) -> & mut Self { self . set_public_key (val) ; self } # [cfg (feature = "AbortSignal")] # [deprecated = "Use `set_signal()` instead."] pub fn signal (& mut self , val : & AbortSignal) -> & mut Self { self . set_signal (val) ; self } }
};
}
