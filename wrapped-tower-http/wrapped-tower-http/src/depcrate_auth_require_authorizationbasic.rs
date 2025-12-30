// Generated macro for Basic (struct)
macro_rules! Depcrate_auth_require_authorizationBasic {
() => {
// Module: crate::auth::require_authorization
// Provides: {"Basic"}
// Dependencies: {}
# [doc = " Type that performs basic authorization."] # [doc = ""] # [doc = " See [`ValidateRequestHeader::basic`] for more details."] pub struct Basic < ResBody > { header_value : HeaderValue , _ty : PhantomData < fn () -> ResBody > , }
};
}
