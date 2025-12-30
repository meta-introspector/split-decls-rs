// Generated macro for AsyncRequireAuthorization (struct)
macro_rules! Depcrate_auth_async_require_authorizationAsyncRequireAuthorization {
() => {
// Module: crate::auth::async_require_authorization
// Provides: {"AsyncRequireAuthorization"}
// Dependencies: {}
# [doc = " Middleware that authorizes all requests using the [`Authorization`] header."] # [doc = ""] # [doc = " See the [module docs](crate::auth::async_require_authorization) for an example."] # [doc = ""] # [doc = " [`Authorization`]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Authorization"] # [derive (Clone , Debug)] pub struct AsyncRequireAuthorization < S , T > { inner : S , auth : T , }
};
}
