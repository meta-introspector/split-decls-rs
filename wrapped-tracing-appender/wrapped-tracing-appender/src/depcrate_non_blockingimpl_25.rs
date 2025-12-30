// Generated macro for impl_25 (impl)
macro_rules! Depcrate_non_blockingimpl_25 {
() => {
// Module: crate::non_blocking
// Provides: {"impl_25"}
// Dependencies: {}
impl WorkerGuard { fn new (handle : JoinHandle < () > , sender : Sender < Msg > , shutdown : Sender < () >) -> Self { WorkerGuard { _guard : Some (handle) , sender , shutdown , } } }
};
}
