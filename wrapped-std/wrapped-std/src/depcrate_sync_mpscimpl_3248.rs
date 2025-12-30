// Generated macro for impl_3248 (impl)
macro_rules! Depcrate_sync_mpscimpl_3248 {
() => {
// Module: crate::sync::mpsc
// Provides: {"impl_3248"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T > Clone for SyncSender < T > { fn clone (& self) -> SyncSender < T > { SyncSender { inner : self . inner . clone () } } }
};
}
