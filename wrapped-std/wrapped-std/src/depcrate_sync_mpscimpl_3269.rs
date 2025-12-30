// Generated macro for impl_3269 (impl)
macro_rules! Depcrate_sync_mpscimpl_3269 {
() => {
// Module: crate::sync::mpsc
// Provides: {"impl_3269"}
// Dependencies: {}
# [stable (feature = "mpsc_recv_timeout_error" , since = "1.15.0")] impl fmt :: Display for RecvTimeoutError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { RecvTimeoutError :: Timeout => "timed out waiting on channel" . fmt (f) , RecvTimeoutError :: Disconnected => "channel is empty and sending half is closed" . fmt (f) , } } }
};
}
