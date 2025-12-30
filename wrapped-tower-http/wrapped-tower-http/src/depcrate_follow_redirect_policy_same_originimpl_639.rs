// Generated macro for impl_639 (impl)
macro_rules! Depcrate_follow_redirect_policy_same_originimpl_639 {
() => {
// Module: crate::follow_redirect::policy::same_origin
// Provides: {"impl_639"}
// Dependencies: {}
impl < B , E > Policy < B , E > for SameOrigin { fn redirect (& mut self , attempt : & Attempt < '_ >) -> Result < Action , E > { if eq_origin (attempt . previous () , attempt . location ()) { Ok (Action :: Follow) } else { Ok (Action :: Stop) } } }
};
}
