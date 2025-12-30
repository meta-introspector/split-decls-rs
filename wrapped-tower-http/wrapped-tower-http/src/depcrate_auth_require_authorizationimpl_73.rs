// Generated macro for impl_73 (impl)
macro_rules! Depcrate_auth_require_authorizationimpl_73 {
() => {
// Module: crate::auth::require_authorization
// Provides: {"impl_73"}
// Dependencies: {}
impl < S , ResBody > ValidateRequestHeader < S , Basic < ResBody > > { # [doc = " Authorize requests using a username and password pair."] # [doc = ""] # [doc = " The `Authorization` header is required to be `Basic {credentials}` where `credentials` is"] # [doc = " `base64_encode(\"{username}:{password}\")`."] # [doc = ""] # [doc = " Since the username and password is sent in clear text it is recommended to use HTTPS/TLS"] # [doc = " with this method. However use of HTTPS/TLS is not enforced by this middleware."] pub fn basic (inner : S , username : & str , value : & str) -> Self where ResBody : Default , { Self :: custom (inner , Basic :: new (username , value)) } }
};
}
