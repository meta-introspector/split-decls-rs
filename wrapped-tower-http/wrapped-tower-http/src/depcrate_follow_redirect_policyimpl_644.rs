// Generated macro for impl_644 (impl)
macro_rules! Depcrate_follow_redirect_policyimpl_644 {
() => {
// Module: crate::follow_redirect::policy
// Provides: {"impl_644"}
// Dependencies: {}
impl < B , E , P > Policy < B , E > for & mut P where P : Policy < B , E > + ? Sized , { fn redirect (& mut self , attempt : & Attempt < '_ >) -> Result < Action , E > { (* * self) . redirect (attempt) } fn on_request (& mut self , request : & mut Request < B >) { (* * self) . on_request (request) } fn clone_body (& self , body : & B) -> Option < B > { (* * self) . clone_body (body) } }
};
}
