// Generated macro for impl_26 (impl)
macro_rules! Depcrate_non_blockingimpl_26 {
() => {
// Module: crate::non_blocking
// Provides: {"impl_26"}
// Dependencies: {}
impl Drop for WorkerGuard { fn drop (& mut self) { match self . sender . send_timeout (Msg :: Shutdown , Duration :: from_millis (100)) { Ok (_) => { let _ = self . shutdown . send_timeout (() , Duration :: from_millis (1000)) ; } Err (SendTimeoutError :: Disconnected (_)) => () , Err (SendTimeoutError :: Timeout (e)) => println ! ("Failed to send shutdown signal to logging worker. Error: {:?}" , e) , } } }
};
}
