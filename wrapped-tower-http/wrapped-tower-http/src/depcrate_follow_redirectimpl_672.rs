// Generated macro for impl_672 (impl)
macro_rules! Depcrate_follow_redirectimpl_672 {
() => {
// Module: crate::follow_redirect
// Provides: {"impl_672"}
// Dependencies: {}
impl < S > FollowRedirect < S > { # [doc = " Create a new [`FollowRedirect`] with a [`Standard`] redirection policy."] pub fn new (inner : S) -> Self { Self :: with_policy (inner , Standard :: default ()) } # [doc = " Returns a new [`Layer`] that wraps services with a `FollowRedirect` middleware."] # [doc = ""] # [doc = " [`Layer`]: tower_layer::Layer"] pub fn layer () -> FollowRedirectLayer { FollowRedirectLayer :: new () } }
};
}
