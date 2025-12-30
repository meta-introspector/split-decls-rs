// Generated macro for impl_222 (impl)
macro_rules! Depcrate_loom_std_atomic_u16impl_222 {
() => {
// Module: crate::loom::std::atomic_u16
// Provides: {"impl_222"}
// Dependencies: {}
impl AtomicU16 { pub (crate) const fn new (val : u16) -> AtomicU16 { let inner = UnsafeCell :: new (std :: sync :: atomic :: AtomicU16 :: new (val)) ; AtomicU16 { inner } } # [doc = " Performs an unsynchronized load."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " All mutations must have happened before the unsynchronized load."] # [doc = " Additionally, there must be no concurrent mutations."] pub (crate) unsafe fn unsync_load (& self) -> u16 { unsafe { core :: ptr :: read (self . inner . get () as * const u16) } } }
};
}
