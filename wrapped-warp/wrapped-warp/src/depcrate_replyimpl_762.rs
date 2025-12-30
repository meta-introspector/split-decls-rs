// Generated macro for impl_762 (impl)
macro_rules! Depcrate_replyimpl_762 {
() => {
// Module: crate::reply
// Provides: {"impl_762"}
// Dependencies: {}
impl < T : Reply > Reply for WithHeader < T > { fn into_response (self) -> Response { let mut res = self . reply . into_response () ; if let Some ((name , value)) = self . header { res . headers_mut () . insert (name , value) ; } res } }
};
}
