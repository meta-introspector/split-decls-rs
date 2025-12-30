// Generated macro for impl_764 (impl)
macro_rules! Depcrate_replyimpl_764 {
() => {
// Module: crate::reply
// Provides: {"impl_764"}
// Dependencies: {}
impl Reply for :: http :: StatusCode { # [inline] fn into_response (self) -> Response { let mut res = Response :: default () ; * res . status_mut () = self ; res } }
};
}
