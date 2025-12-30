// Generated macro for impl_512 (impl)
macro_rules! Depcrate_features_gen_AuthenticatorSelectionCriteriaimpl_512 {
() => {
// Module: crate::features::gen_AuthenticatorSelectionCriteria
// Provides: {"impl_512"}
// Dependencies: {}
impl AuthenticatorSelectionCriteria { # [doc = "Construct a new `AuthenticatorSelectionCriteria`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [cfg (feature = "AuthenticatorAttachment")] # [deprecated = "Use `set_authenticator_attachment()` instead."] pub fn authenticator_attachment (& mut self , val : AuthenticatorAttachment) -> & mut Self { self . set_authenticator_attachment (val) ; self } # [deprecated = "Use `set_require_resident_key()` instead."] pub fn require_resident_key (& mut self , val : bool) -> & mut Self { self . set_require_resident_key (val) ; self } # [deprecated = "Use `set_resident_key()` instead."] pub fn resident_key (& mut self , val : & str) -> & mut Self { self . set_resident_key (val) ; self } # [cfg (feature = "UserVerificationRequirement")] # [deprecated = "Use `set_user_verification()` instead."] pub fn user_verification (& mut self , val : UserVerificationRequirement) -> & mut Self { self . set_user_verification (val) ; self } }
};
}
