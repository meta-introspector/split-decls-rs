// Generated macro for impl_755 (impl)
macro_rules! Depcrate_replyimpl_755 {
() => {
// Module: crate::reply
// Provides: {"impl_755"}
// Dependencies: {}
impl < T : Reply + ? Sized > Reply for Box < T > { fn into_response (self) -> Response { self . boxed_into_response (Internal) } }
};
}
