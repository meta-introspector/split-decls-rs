// Generated macro for impl_3345 (impl)
macro_rules! Depcrate_sync_lazy_lockimpl_3345 {
() => {
// Module: crate::sync::lazy_lock
// Provides: {"impl_3345"}
// Dependencies: {}
# [stable (feature = "lazy_cell" , since = "1.80.0")] impl < T : Default > Default for LazyLock < T > { # [doc = " Creates a new lazy value using `Default` as the initializing function."] # [inline] fn default () -> LazyLock < T > { LazyLock :: new (T :: default) } }
};
}
