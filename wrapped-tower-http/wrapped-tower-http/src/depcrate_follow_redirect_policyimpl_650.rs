// Generated macro for impl_650 (impl)
macro_rules! Depcrate_follow_redirect_policyimpl_650 {
() => {
// Module: crate::follow_redirect::policy
// Provides: {"impl_650"}
// Dependencies: {}
impl < 'a > Attempt < 'a > { # [doc = " Returns the redirection response."] pub fn status (& self) -> StatusCode { self . status } # [doc = " Returns the destination URI of the redirection."] pub fn location (& self) -> & 'a Uri { self . location } # [doc = " Returns the URI of the original request."] pub fn previous (& self) -> & 'a Uri { self . previous } }
};
}
