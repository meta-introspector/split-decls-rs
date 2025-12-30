// Generated macro for impl_223 (impl)
macro_rules! Depcrate_loom_std_atomic_u16impl_223 {
() => {
// Module: crate::loom::std::atomic_u16
// Provides: {"impl_223"}
// Dependencies: {}
impl Deref for AtomicU16 { type Target = std :: sync :: atomic :: AtomicU16 ; fn deref (& self) -> & Self :: Target { unsafe { & * self . inner . get () } } }
};
}
