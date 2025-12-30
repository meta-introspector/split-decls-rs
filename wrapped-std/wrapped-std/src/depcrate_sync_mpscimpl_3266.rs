// Generated macro for impl_3266 (impl)
macro_rules! Depcrate_sync_mpscimpl_3266 {
() => {
// Module: crate::sync::mpsc
// Provides: {"impl_3266"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl fmt :: Display for TryRecvError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { TryRecvError :: Empty => "receiving on an empty channel" . fmt (f) , TryRecvError :: Disconnected => "receiving on a closed channel" . fmt (f) , } } }
};
}
