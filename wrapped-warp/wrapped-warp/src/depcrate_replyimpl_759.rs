// Generated macro for impl_759 (impl)
macro_rules! Depcrate_replyimpl_759 {
() => {
// Module: crate::reply
// Provides: {"impl_759"}
// Dependencies: {}
impl < T : Reply > Reply for WithStatus < T > { fn into_response (self) -> Response { let mut res = self . reply . into_response () ; * res . status_mut () = self . status ; res } }
};
}
