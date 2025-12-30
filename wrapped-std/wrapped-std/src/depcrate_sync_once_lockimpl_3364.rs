// Generated macro for impl_3364 (impl)
macro_rules! Depcrate_sync_once_lockimpl_3364 {
() => {
// Module: crate::sync::once_lock
// Provides: {"impl_3364"}
// Dependencies: {}
# [stable (feature = "once_cell" , since = "1.70.0")] impl < T > Default for OnceLock < T > { # [doc = " Creates a new uninitialized cell."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::OnceLock;"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     assert_eq!(OnceLock::<()>::new(), OnceLock::default());"] # [doc = " }"] # [doc = " ```"] # [inline] fn default () -> OnceLock < T > { OnceLock :: new () } }
};
}
