// Generated macro for impl_32 (impl)
macro_rules! Depcrate_spin_muteximpl_32 {
() => {
// Module: crate::spin::mutex
// Provides: {"impl_32"}
// Dependencies: {}
impl < 'a , T : ? Sized > Drop for MutexGuard < 'a , T > { # [doc = " The dropping of the MutexGuard will release the lock it was created from."] fn drop (& mut self) { self . lock . store (false , Ordering :: Release) ; } }
};
}
