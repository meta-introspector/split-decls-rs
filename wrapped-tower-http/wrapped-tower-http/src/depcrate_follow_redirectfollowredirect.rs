// Generated macro for FollowRedirect (struct)
macro_rules! Depcrate_follow_redirectFollowRedirect {
() => {
// Module: crate::follow_redirect
// Provides: {"FollowRedirect"}
// Dependencies: {}
# [doc = " Middleware that retries requests with a [`Service`] to follow redirection responses."] # [doc = ""] # [doc = " See the [module docs](self) for more details."] # [derive (Clone , Copy , Debug)] pub struct FollowRedirect < S , P = Standard > { inner : S , policy : P , }
};
}
