// Generated macro for impl_601 (impl)
macro_rules! Depcrate_follow_redirect_policy_clone_body_fnimpl_601 {
() => {
// Module: crate::follow_redirect::policy::clone_body_fn
// Provides: {"impl_601"}
// Dependencies: {}
impl < F , B , E > Policy < B , E > for CloneBodyFn < F > where F : Fn (& B) -> Option < B > , { fn redirect (& mut self , _ : & Attempt < '_ >) -> Result < Action , E > { Ok (Action :: Follow) } fn clone_body (& self , body : & B) -> Option < B > { (self . f) (body) } }
};
}
