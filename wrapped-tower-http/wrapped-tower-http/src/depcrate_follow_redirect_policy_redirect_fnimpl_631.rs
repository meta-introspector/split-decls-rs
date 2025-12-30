// Generated macro for impl_631 (impl)
macro_rules! Depcrate_follow_redirect_policy_redirect_fnimpl_631 {
() => {
// Module: crate::follow_redirect::policy::redirect_fn
// Provides: {"impl_631"}
// Dependencies: {}
impl < B , E , F > Policy < B , E > for RedirectFn < F > where F : FnMut (& Attempt < '_ >) -> Result < Action , E > , { fn redirect (& mut self , attempt : & Attempt < '_ >) -> Result < Action , E > { (self . f) (attempt) } }
};
}
