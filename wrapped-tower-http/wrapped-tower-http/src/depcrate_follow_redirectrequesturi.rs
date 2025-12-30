// Generated macro for RequestUri (struct)
macro_rules! Depcrate_follow_redirectRequestUri {
() => {
// Module: crate::follow_redirect
// Provides: {"RequestUri"}
// Dependencies: {}
# [doc = " Response [`Extensions`][http::Extensions] value that represents the effective request URI of"] # [doc = " a response returned by a [`FollowRedirect`] middleware."] # [doc = ""] # [doc = " The value differs from the original request's effective URI if the middleware has followed"] # [doc = " redirections."] # [derive (Clone)] pub struct RequestUri (pub Uri) ;
};
}
