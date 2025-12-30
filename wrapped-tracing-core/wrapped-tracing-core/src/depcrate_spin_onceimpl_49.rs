// Generated macro for impl_49 (impl)
macro_rules! Depcrate_spin_onceimpl_49 {
() => {
// Module: crate::spin::once
// Provides: {"impl_49"}
// Dependencies: {}
impl < 'a > Drop for Finish < 'a > { fn drop (& mut self) { if self . panicked { self . state . store (PANICKED , Ordering :: SeqCst) ; } } }
};
}
