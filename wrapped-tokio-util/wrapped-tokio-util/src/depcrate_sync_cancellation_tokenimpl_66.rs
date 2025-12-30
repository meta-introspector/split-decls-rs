// Generated macro for impl_66 (impl)
macro_rules! Depcrate_sync_cancellation_tokenimpl_66 {
() => {
// Module: crate::sync::cancellation_token
// Provides: {"impl_66"}
// Dependencies: {}
impl < 'a > Future for WaitForCancellationFuture < 'a > { type Output = () ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < () > { let mut this = self . project () ; loop { if this . cancellation_token . is_cancelled () { return Poll :: Ready (()) ; } if this . future . as_mut () . poll (cx) . is_pending () { return Poll :: Pending ; } this . future . set (this . cancellation_token . inner . notified ()) ; } } }
};
}
