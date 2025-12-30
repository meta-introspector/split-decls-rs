// Generated macro for impl_76 (impl)
macro_rules! Depcrate_auth_require_authorizationimpl_76 {
() => {
// Module: crate::auth::require_authorization
// Provides: {"impl_76"}
// Dependencies: {}
impl < ResBody > ValidateRequestHeaderLayer < Bearer < ResBody > > { # [doc = " Authorize requests using a \"bearer token\". Commonly used for OAuth 2."] # [doc = ""] # [doc = " The `Authorization` header is required to be `Bearer {token}`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the token is not a valid [`HeaderValue`]."] pub fn bearer (token : & str) -> Self where ResBody : Default , { Self :: custom (Bearer :: new (token)) } }
};
}
