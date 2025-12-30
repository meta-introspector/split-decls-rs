// Generated macro for impl_617 (impl)
macro_rules! Depcrate_follow_redirect_policy_limitedimpl_617 {
() => {
// Module: crate::follow_redirect::policy::limited
// Provides: {"impl_617"}
// Dependencies: {}
impl < B , E > Policy < B , E > for Limited { fn redirect (& mut self , _ : & Attempt < '_ >) -> Result < Action , E > { if self . remaining > 0 { self . remaining -= 1 ; Ok (Action :: Follow) } else { Ok (Action :: Stop) } } }
};
}
