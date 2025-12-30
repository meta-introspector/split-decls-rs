// Generated macro for impl_253 (impl)
macro_rules! Depcrate_loom_std_atomic_usizeimpl_253 {
() => {
// Module: crate::loom::std::atomic_usize
// Provides: {"impl_253"}
// Dependencies: {}
impl ops :: Deref for AtomicUsize { type Target = std :: sync :: atomic :: AtomicUsize ; fn deref (& self) -> & Self :: Target { unsafe { & * self . inner . get () } } }
};
}
