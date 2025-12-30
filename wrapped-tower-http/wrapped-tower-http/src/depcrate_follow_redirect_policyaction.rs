// Generated macro for Action (enum)
macro_rules! Depcrate_follow_redirect_policyAction {
() => {
// Module: crate::follow_redirect::policy
// Provides: {"Action"}
// Dependencies: {}
# [doc = " A value returned by [`Policy::redirect`] which indicates the action"] # [doc = " [`FollowRedirect`][super::FollowRedirect] should take for a redirection response."] # [derive (Clone , Copy , Debug)] pub enum Action { # [doc = " Follow the redirection."] Follow , # [doc = " Do not follow the redirection, and return the redirection response as-is."] Stop , }
};
}
