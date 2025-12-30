// Generated macro for impl_673 (impl)
macro_rules! Depcrate_follow_redirectimpl_673 {
() => {
// Module: crate::follow_redirect
// Provides: {"impl_673"}
// Dependencies: {}
impl < S , P > FollowRedirect < S , P > where P : Clone , { # [doc = " Create a new [`FollowRedirect`] with the given redirection [`Policy`]."] pub fn with_policy (inner : S , policy : P) -> Self { FollowRedirect { inner , policy } } # [doc = " Returns a new [`Layer`] that wraps services with a `FollowRedirect` middleware"] # [doc = " with the given redirection [`Policy`]."] # [doc = ""] # [doc = " [`Layer`]: tower_layer::Layer"] pub fn layer_with_policy (policy : P) -> FollowRedirectLayer < P > { FollowRedirectLayer :: with_policy (policy) } define_inner_service_accessors ! () ; }
};
}
