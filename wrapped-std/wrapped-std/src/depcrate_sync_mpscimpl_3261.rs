// Generated macro for impl_3261 (impl)
macro_rules! Depcrate_sync_mpscimpl_3261 {
() => {
// Module: crate::sync::mpsc
// Provides: {"impl_3261"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T > fmt :: Display for TrySendError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { TrySendError :: Full (..) => "sending on a full channel" . fmt (f) , TrySendError :: Disconnected (..) => "sending on a closed channel" . fmt (f) , } } }
};
}
