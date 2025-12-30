// Generated macro for impl_30 (impl)
macro_rules! Depcrate_spin_muteximpl_30 {
() => {
// Module: crate::spin::mutex
// Provides: {"impl_30"}
// Dependencies: {}
impl < 'a , T : ? Sized > Deref for MutexGuard < 'a , T > { type Target = T ; fn deref < 'b > (& 'b self) -> & 'b T { & * self . data } }
};
}
