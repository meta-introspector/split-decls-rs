// Generated macro for impl_69 (impl)
macro_rules! Depcrate_sync_cancellation_tokenimpl_69 {
() => {
// Module: crate::sync::cancellation_token
// Provides: {"impl_69"}
// Dependencies: {}
impl Future for WaitForCancellationFutureOwned { type Output = () ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < () > { let mut this = self . project () ; loop { if this . cancellation_token . is_cancelled () { return Poll :: Ready (()) ; } if this . future . as_mut () . poll (cx) . is_pending () { return Poll :: Pending ; } this . future . set (MaybeDangling :: new (unsafe { Self :: new_future (this . cancellation_token) })) ; } } }
};
}
