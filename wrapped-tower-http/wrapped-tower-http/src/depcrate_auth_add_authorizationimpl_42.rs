// Generated macro for impl_42 (impl)
macro_rules! Depcrate_auth_add_authorizationimpl_42 {
() => {
// Module: crate::auth::add_authorization
// Provides: {"impl_42"}
// Dependencies: {}
impl AddAuthorizationLayer { # [doc = " Authorize requests using a username and password pair."] # [doc = ""] # [doc = " The `Authorization` header will be set to `Basic {credentials}` where `credentials` is"] # [doc = " `base64_encode(\"{username}:{password}\")`."] # [doc = ""] # [doc = " Since the username and password is sent in clear text it is recommended to use HTTPS/TLS"] # [doc = " with this method. However use of HTTPS/TLS is not enforced by this middleware."] pub fn basic (username : & str , password : & str) -> Self { let encoded = BASE64 . encode (format ! ("{}:{}" , username , password)) ; let value = HeaderValue :: try_from (format ! ("Basic {}" , encoded)) . unwrap () ; Self { value } } # [doc = " Authorize requests using a \"bearer token\". Commonly used for OAuth 2."] # [doc = ""] # [doc = " The `Authorization` header will be set to `Bearer {token}`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the token is not a valid [`HeaderValue`]."] pub fn bearer (token : & str) -> Self { let value = HeaderValue :: try_from (format ! ("Bearer {}" , token)) . expect ("token is not valid header") ; Self { value } } # [doc = " Mark the header as [sensitive]."] # [doc = ""] # [doc = " This can for example be used to hide the header value from logs."] # [doc = ""] # [doc = " [sensitive]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive"] # [allow (clippy :: wrong_self_convention)] pub fn as_sensitive (mut self , sensitive : bool) -> Self { self . value . set_sensitive (sensitive) ; self } }
};
}
