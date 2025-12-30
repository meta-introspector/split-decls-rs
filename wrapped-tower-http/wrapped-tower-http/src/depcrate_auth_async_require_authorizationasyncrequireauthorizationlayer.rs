// Generated macro for AsyncRequireAuthorizationLayer (struct)
macro_rules! Depcrate_auth_async_require_authorizationAsyncRequireAuthorizationLayer {
() => {
// Module: crate::auth::async_require_authorization
// Provides: {"AsyncRequireAuthorizationLayer"}
// Dependencies: {}
# [doc = " Layer that applies [`AsyncRequireAuthorization`] which authorizes all requests using the"] # [doc = " [`Authorization`] header."] # [doc = ""] # [doc = " See the [module docs](crate::auth::async_require_authorization) for an example."] # [doc = ""] # [doc = " [`Authorization`]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Authorization"] # [derive (Debug , Clone)] pub struct AsyncRequireAuthorizationLayer < T > { auth : T , }
};
}
