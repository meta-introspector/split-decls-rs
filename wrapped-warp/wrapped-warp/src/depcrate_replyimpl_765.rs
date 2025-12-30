// Generated macro for impl_765 (impl)
macro_rules! Depcrate_replyimpl_765 {
() => {
// Module: crate::reply
// Provides: {"impl_765"}
// Dependencies: {}
impl Reply for :: http :: Error { # [inline] fn into_response (self) -> Response { tracing :: error ! ("reply error: {:?}" , self) ; StatusCode :: INTERNAL_SERVER_ERROR . into_response () } }
};
}
