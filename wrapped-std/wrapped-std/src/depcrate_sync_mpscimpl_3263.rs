// Generated macro for impl_3263 (impl)
macro_rules! Depcrate_sync_mpscimpl_3263 {
() => {
// Module: crate::sync::mpsc
// Provides: {"impl_3263"}
// Dependencies: {}
# [stable (feature = "mpsc_error_conversions" , since = "1.24.0")] impl < T > From < SendError < T > > for TrySendError < T > { # [doc = " Converts a `SendError<T>` into a `TrySendError<T>`."] # [doc = ""] # [doc = " This conversion always returns a `TrySendError::Disconnected` containing the data in the `SendError<T>`."] # [doc = ""] # [doc = " No data is allocated on the heap."] fn from (err : SendError < T >) -> TrySendError < T > { match err { SendError (t) => TrySendError :: Disconnected (t) , } } }
};
}
