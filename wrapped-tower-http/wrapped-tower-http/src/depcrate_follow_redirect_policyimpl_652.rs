// Generated macro for impl_652 (impl)
macro_rules! Depcrate_follow_redirect_policyimpl_652 {
() => {
// Module: crate::follow_redirect::policy
// Provides: {"impl_652"}
// Dependencies: {}
impl Action { # [doc = " Returns `true` if the `Action` is a `Follow` value."] pub fn is_follow (& self) -> bool { if let Action :: Follow = self { true } else { false } } # [doc = " Returns `true` if the `Action` is a `Stop` value."] pub fn is_stop (& self) -> bool { if let Action :: Stop = self { true } else { false } } }
};
}
