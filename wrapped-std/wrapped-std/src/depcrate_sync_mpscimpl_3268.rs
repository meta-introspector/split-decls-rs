// Generated macro for impl_3268 (impl)
macro_rules! Depcrate_sync_mpscimpl_3268 {
() => {
// Module: crate::sync::mpsc
// Provides: {"impl_3268"}
// Dependencies: {}
# [stable (feature = "mpsc_error_conversions" , since = "1.24.0")] impl From < RecvError > for TryRecvError { # [doc = " Converts a `RecvError` into a `TryRecvError`."] # [doc = ""] # [doc = " This conversion always returns `TryRecvError::Disconnected`."] # [doc = ""] # [doc = " No data is allocated on the heap."] fn from (err : RecvError) -> TryRecvError { match err { RecvError => TryRecvError :: Disconnected , } } }
};
}
