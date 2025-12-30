// Generated macro for impl_57 (impl)
macro_rules! Depcrateimpl_57 {
() => {
// Module: crate
// Provides: {"impl_57"}
// Dependencies: {}
# [cfg (feature = "malloc_size_of")] impl < T > MallocShallowSizeOf for ThinVec < T > { fn shallow_size_of (& self , ops : & mut MallocSizeOfOps) -> usize { if self . capacity () == 0 { return 0 ; } assert_eq ! (std :: mem :: size_of ::< Self > () , std :: mem :: size_of ::<* const () > ()) ; unsafe { ops . malloc_size_of (* (self as * const Self as * const * const ())) } } }
};
}
