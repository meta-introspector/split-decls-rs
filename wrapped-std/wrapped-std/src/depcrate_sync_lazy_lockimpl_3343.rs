// Generated macro for impl_3343 (impl)
macro_rules! Depcrate_sync_lazy_lockimpl_3343 {
() => {
// Module: crate::sync::lazy_lock
// Provides: {"impl_3343"}
// Dependencies: {}
# [stable (feature = "lazy_cell" , since = "1.80.0")] impl < T , F : FnOnce () -> T > Deref for LazyLock < T , F > { type Target = T ; # [doc = " Dereferences the value."] # [doc = ""] # [doc = " This method will block the calling thread if another initialization"] # [doc = " routine is currently running."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If the initialization closure panics (the one that is passed to the [`new()`] method), the"] # [doc = " panic is propagated to the caller, and the lock becomes poisoned. This will cause all future"] # [doc = " accesses of the lock (via [`force()`] or a dereference) to panic."] # [doc = ""] # [doc = " [`new()`]: LazyLock::new"] # [doc = " [`force()`]: LazyLock::force"] # [inline] fn deref (& self) -> & T { LazyLock :: force (self) } }
};
}
