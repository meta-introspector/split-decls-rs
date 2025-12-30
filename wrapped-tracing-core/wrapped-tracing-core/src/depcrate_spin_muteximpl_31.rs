// Generated macro for impl_31 (impl)
macro_rules! Depcrate_spin_muteximpl_31 {
() => {
// Module: crate::spin::mutex
// Provides: {"impl_31"}
// Dependencies: {}
impl < 'a , T : ? Sized > DerefMut for MutexGuard < 'a , T > { fn deref_mut < 'b > (& 'b mut self) -> & 'b mut T { & mut * self . data } }
};
}
