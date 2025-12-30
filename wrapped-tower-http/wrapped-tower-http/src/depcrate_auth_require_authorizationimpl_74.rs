// Generated macro for impl_74 (impl)
macro_rules! Depcrate_auth_require_authorizationimpl_74 {
() => {
// Module: crate::auth::require_authorization
// Provides: {"impl_74"}
// Dependencies: {}
impl < ResBody > ValidateRequestHeaderLayer < Basic < ResBody > > { # [doc = " Authorize requests using a username and password pair."] # [doc = ""] # [doc = " The `Authorization` header is required to be `Basic {credentials}` where `credentials` is"] # [doc = " `base64_encode(\"{username}:{password}\")`."] # [doc = ""] # [doc = " Since the username and password is sent in clear text it is recommended to use HTTPS/TLS"] # [doc = " with this method. However use of HTTPS/TLS is not enforced by this middleware."] pub fn basic (username : & str , password : & str) -> Self where ResBody : Default , { Self :: custom (Basic :: new (username , password)) } }
};
}
