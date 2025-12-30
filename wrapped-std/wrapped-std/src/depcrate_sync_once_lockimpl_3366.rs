// Generated macro for impl_3366 (impl)
macro_rules! Depcrate_sync_once_lockimpl_3366 {
() => {
// Module: crate::sync::once_lock
// Provides: {"impl_3366"}
// Dependencies: {}
# [stable (feature = "once_cell" , since = "1.70.0")] impl < T : Clone > Clone for OnceLock < T > { # [inline] fn clone (& self) -> OnceLock < T > { let cell = Self :: new () ; if let Some (value) = self . get () { match cell . set (value . clone ()) { Ok (()) => () , Err (_) => unreachable ! () , } } cell } }
};
}
