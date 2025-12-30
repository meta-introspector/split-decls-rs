// Generated macro for AddAuthorization (struct)
macro_rules! Depcrate_auth_add_authorizationAddAuthorization {
() => {
// Module: crate::auth::add_authorization
// Provides: {"AddAuthorization"}
// Dependencies: {}
# [doc = " Middleware that adds authorization all requests using the [`Authorization`] header."] # [doc = ""] # [doc = " See the [module docs](crate::auth::add_authorization) for an example."] # [doc = ""] # [doc = " You can also use [`SetRequestHeader`] if you have a use case that isn't supported by this"] # [doc = " middleware."] # [doc = ""] # [doc = " [`Authorization`]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Authorization"] # [doc = " [`SetRequestHeader`]: crate::set_header::SetRequestHeader"] # [derive (Debug , Clone)] pub struct AddAuthorization < S > { inner : S , value : HeaderValue , }
};
}
