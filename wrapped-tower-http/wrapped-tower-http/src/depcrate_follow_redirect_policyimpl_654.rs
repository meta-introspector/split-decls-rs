// Generated macro for impl_654 (impl)
macro_rules! Depcrate_follow_redirect_policyimpl_654 {
() => {
// Module: crate::follow_redirect::policy
// Provides: {"impl_654"}
// Dependencies: {}
impl < B , E > Policy < B , E > for Result < Action , E > where E : Clone , { fn redirect (& mut self , _ : & Attempt < '_ >) -> Result < Action , E > { self . clone () } }
};
}
