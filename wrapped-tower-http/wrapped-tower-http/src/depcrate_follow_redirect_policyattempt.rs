// Generated macro for Attempt (struct)
macro_rules! Depcrate_follow_redirect_policyAttempt {
() => {
// Module: crate::follow_redirect::policy
// Provides: {"Attempt"}
// Dependencies: {}
# [doc = " A type that holds information on a redirection attempt."] pub struct Attempt < 'a > { pub (crate) status : StatusCode , pub (crate) location : & 'a Uri , pub (crate) previous : & 'a Uri , }
};
}
