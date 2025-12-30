// Generated macro for impl_235 (impl)
macro_rules! Depcrate_loom_std_atomic_u32impl_235 {
() => {
// Module: crate::loom::std::atomic_u32
// Provides: {"impl_235"}
// Dependencies: {}
impl AtomicU32 { pub (crate) const fn new (val : u32) -> AtomicU32 { let inner = UnsafeCell :: new (std :: sync :: atomic :: AtomicU32 :: new (val)) ; AtomicU32 { inner } } # [doc = " Performs an unsynchronized load."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " All mutations must have happened before the unsynchronized load."] # [doc = " Additionally, there must be no concurrent mutations."] pub (crate) unsafe fn unsync_load (& self) -> u32 { unsafe { core :: ptr :: read (self . inner . get () as * const u32) } } }
};
}
