// Generated macro for impl_28 (impl)
macro_rules! Depcrate_flagsimpl_28 {
() => {
// Module: crate::flags
// Provides: {"impl_28"}
// Dependencies: {}
impl Dist { pub (crate) fn allocator (& self) -> Malloc { if self . mimalloc { Malloc :: Mimalloc } else if self . jemalloc { Malloc :: Jemalloc } else if self . enable_profiling { Malloc :: Dhat } else { Malloc :: System } } }
};
}
