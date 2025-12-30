// Generated macro for cancelable (function)
macro_rules! Depcrate_ready_cache_cachecancelable {
() => {
// Module: crate::ready_cache::cache
// Provides: {"cancelable"}
// Dependencies: {}
# [doc = " Creates a cancelation sender and receiver."] # [doc = ""] # [doc = " A `tokio::sync::oneshot` is NOT used, as a `Receiver` is not guaranteed to"] # [doc = " observe results as soon as a `Sender` fires. Using an `AtomicBool` allows"] # [doc = " the state to be observed as soon as the cancelation is triggered."] fn cancelable () -> CancelPair { let cx = Arc :: new (Cancel { waker : AtomicWaker :: new () , canceled : AtomicBool :: new (false) , }) ; (CancelTx (cx . clone ()) , CancelRx (cx)) }
};
}
