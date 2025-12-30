// Generated macro for impl_496 (impl)
macro_rules! Depcrate_ready_cache_cacheimpl_496 {
() => {
// Module: crate::ready_cache::cache
// Provides: {"impl_496"}
// Dependencies: {}
impl < K , S , Req > Future for Pending < K , S , Req > where S : Service < Req > , { type Output = Result < (K , S , CancelRx) , PendingError < K , S :: Error > > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; let CancelRx (cancel) = this . cancel . as_mut () . expect ("polled after complete") ; if cancel . canceled . load (Ordering :: SeqCst) { let key = this . key . take () . expect ("polled after complete") ; return Err (PendingError :: Canceled (key)) . into () ; } match this . ready . as_mut () . expect ("polled after ready") . poll_ready (cx) { Poll :: Pending => { let CancelRx (cancel) = this . cancel . as_mut () . expect ("polled after complete") ; cancel . waker . register (cx . waker ()) ; assert ! (! cancel . canceled . load (Ordering :: SeqCst) , "cancelation cannot be notified while polling a pending service") ; Poll :: Pending } Poll :: Ready (Ok (())) => { let key = this . key . take () . expect ("polled after complete") ; let cancel = this . cancel . take () . expect ("polled after complete") ; Ok ((key , this . ready . take () . expect ("polled after ready") , cancel)) . into () } Poll :: Ready (Err (e)) => { let key = this . key . take () . expect ("polled after compete") ; Err (PendingError :: Inner (key , e)) . into () } } } }
};
}
