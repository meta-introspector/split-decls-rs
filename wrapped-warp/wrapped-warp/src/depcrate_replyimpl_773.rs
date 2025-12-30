// Generated macro for impl_773 (impl)
macro_rules! Depcrate_replyimpl_773 {
() => {
// Module: crate::reply
// Provides: {"impl_773"}
// Dependencies: {}
impl < T , U > Reply for Either < T , U > where T : Reply , U : Reply , { # [inline] fn into_response (self) -> Response { match self { Either :: A (a) => a . into_response () , Either :: B (b) => b . into_response () , } } }
};
}
