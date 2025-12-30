// Generated macro for impl_3271 (impl)
macro_rules! Depcrate_sync_mpscimpl_3271 {
() => {
// Module: crate::sync::mpsc
// Provides: {"impl_3271"}
// Dependencies: {}
# [stable (feature = "mpsc_error_conversions" , since = "1.24.0")] impl From < RecvError > for RecvTimeoutError { # [doc = " Converts a `RecvError` into a `RecvTimeoutError`."] # [doc = ""] # [doc = " This conversion always returns `RecvTimeoutError::Disconnected`."] # [doc = ""] # [doc = " No data is allocated on the heap."] fn from (err : RecvError) -> RecvTimeoutError { match err { RecvError => RecvTimeoutError :: Disconnected , } } }
};
}
