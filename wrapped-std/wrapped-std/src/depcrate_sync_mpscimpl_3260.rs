// Generated macro for impl_3260 (impl)
macro_rules! Depcrate_sync_mpscimpl_3260 {
() => {
// Module: crate::sync::mpsc
// Provides: {"impl_3260"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T > fmt :: Debug for TrySendError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { TrySendError :: Full (..) => "Full(..)" . fmt (f) , TrySendError :: Disconnected (..) => "Disconnected(..)" . fmt (f) , } } }
};
}
