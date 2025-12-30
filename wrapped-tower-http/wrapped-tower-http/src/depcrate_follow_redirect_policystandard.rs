// Generated macro for Standard (type)
macro_rules! Depcrate_follow_redirect_policyStandard {
() => {
// Module: crate::follow_redirect::policy
// Provides: {"Standard"}
// Dependencies: {}
# [doc = " A redirection [`Policy`] with a reasonable set of standard behavior."] # [doc = ""] # [doc = " This policy limits the number of successive redirections ([`Limited`])"] # [doc = " and removes credentials from requests in cross-origin redirections ([`FilterCredentials`])."] pub type Standard = And < Limited , FilterCredentials > ;
};
}
