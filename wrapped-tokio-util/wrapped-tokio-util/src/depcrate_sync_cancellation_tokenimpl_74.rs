// Generated macro for impl_74 (impl)
macro_rules! Depcrate_sync_cancellation_tokenimpl_74 {
() => {
// Module: crate::sync::cancellation_token
// Provides: {"impl_74"}
// Dependencies: {}
impl < F : Future > Future for RunUntilCancelledFutureOwned < F > { type Output = Option < F :: Output > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; if let Poll :: Ready (res) = this . future . poll (cx) { Poll :: Ready (Some (res)) } else if this . cancellation . poll (cx) . is_ready () { Poll :: Ready (None) } else { Poll :: Pending } } }
};
}
