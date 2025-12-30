// Generated macro for impl_670 (impl)
macro_rules! Depcrate_follow_redirectimpl_670 {
() => {
// Module: crate::follow_redirect
// Provides: {"impl_670"}
// Dependencies: {}
impl < S , P > Layer < S > for FollowRedirectLayer < P > where S : Clone , P : Clone , { type Service = FollowRedirect < S , P > ; fn layer (& self , inner : S) -> Self :: Service { FollowRedirect :: with_policy (inner , self . policy . clone ()) } }
};
}
