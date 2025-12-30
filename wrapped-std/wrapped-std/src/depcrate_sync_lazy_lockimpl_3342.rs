// Generated macro for impl_3342 (impl)
macro_rules! Depcrate_sync_lazy_lockimpl_3342 {
() => {
// Module: crate::sync::lazy_lock
// Provides: {"impl_3342"}
// Dependencies: {}
# [stable (feature = "lazy_cell" , since = "1.80.0")] impl < T , F > Drop for LazyLock < T , F > { fn drop (& mut self) { match self . once . state () { ExclusiveState :: Incomplete => unsafe { ManuallyDrop :: drop (& mut self . data . get_mut () . f) } , ExclusiveState :: Complete => unsafe { ManuallyDrop :: drop (& mut self . data . get_mut () . value) } , ExclusiveState :: Poisoned => { } } } }
};
}
