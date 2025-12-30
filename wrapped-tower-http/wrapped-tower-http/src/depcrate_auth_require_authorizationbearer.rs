// Generated macro for Bearer (struct)
macro_rules! Depcrate_auth_require_authorizationBearer {
() => {
// Module: crate::auth::require_authorization
// Provides: {"Bearer"}
// Dependencies: {}
# [doc = " Type that performs \"bearer token\" authorization."] # [doc = ""] # [doc = " See [`ValidateRequestHeader::bearer`] for more details."] pub struct Bearer < ResBody > { header_value : HeaderValue , _ty : PhantomData < fn () -> ResBody > , }
};
}
