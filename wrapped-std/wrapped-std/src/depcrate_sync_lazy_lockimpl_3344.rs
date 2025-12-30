// Generated macro for impl_3344 (impl)
macro_rules! Depcrate_sync_lazy_lockimpl_3344 {
() => {
// Module: crate::sync::lazy_lock
// Provides: {"impl_3344"}
// Dependencies: {}
# [stable (feature = "lazy_deref_mut" , since = "1.89.0")] impl < T , F : FnOnce () -> T > DerefMut for LazyLock < T , F > { # [doc = " # Panics"] # [doc = ""] # [doc = " If the initialization closure panics (the one that is passed to the [`new()`] method), the"] # [doc = " panic is propagated to the caller, and the lock becomes poisoned. This will cause all future"] # [doc = " accesses of the lock (via [`force()`] or a dereference) to panic."] # [doc = ""] # [doc = " [`new()`]: LazyLock::new"] # [doc = " [`force()`]: LazyLock::force"] # [inline] fn deref_mut (& mut self) -> & mut T { LazyLock :: force_mut (self) } }
};
}
