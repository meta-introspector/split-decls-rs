// Generated macro for impl_59 (impl)
macro_rules! Depcrate_auth_async_require_authorizationimpl_59 {
() => {
// Module: crate::auth::async_require_authorization
// Provides: {"impl_59"}
// Dependencies: {}
impl < S , T > AsyncRequireAuthorization < S , T > { # [doc = " Authorize requests using a custom scheme."] # [doc = ""] # [doc = " The `Authorization` header is required to have the value provided."] pub fn new (inner : S , auth : T) -> AsyncRequireAuthorization < S , T > { Self { inner , auth } } # [doc = " Returns a new [`Layer`] that wraps services with an [`AsyncRequireAuthorizationLayer`]"] # [doc = " middleware."] # [doc = ""] # [doc = " [`Layer`]: tower_layer::Layer"] pub fn layer (auth : T) -> AsyncRequireAuthorizationLayer < T > { AsyncRequireAuthorizationLayer :: new (auth) } }
};
}
