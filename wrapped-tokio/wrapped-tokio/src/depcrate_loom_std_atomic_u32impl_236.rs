// Generated macro for impl_236 (impl)
macro_rules! Depcrate_loom_std_atomic_u32impl_236 {
() => {
// Module: crate::loom::std::atomic_u32
// Provides: {"impl_236"}
// Dependencies: {}
impl Deref for AtomicU32 { type Target = std :: sync :: atomic :: AtomicU32 ; fn deref (& self) -> & Self :: Target { unsafe { & * self . inner . get () } } }
};
}
