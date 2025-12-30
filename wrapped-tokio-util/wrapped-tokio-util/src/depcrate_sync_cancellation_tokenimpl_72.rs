// Generated macro for impl_72 (impl)
macro_rules! Depcrate_sync_cancellation_tokenimpl_72 {
() => {
// Module: crate::sync::cancellation_token
// Provides: {"impl_72"}
// Dependencies: {}
impl < 'a , F : Future > Future for RunUntilCancelledFuture < 'a , F > { type Output = Option < F :: Output > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; if let Poll :: Ready (res) = this . future . poll (cx) { Poll :: Ready (Some (res)) } else if this . cancellation . poll (cx) . is_ready () { Poll :: Ready (None) } else { Poll :: Pending } } }
};
}
