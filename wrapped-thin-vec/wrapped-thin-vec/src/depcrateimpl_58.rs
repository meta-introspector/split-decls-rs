// Generated macro for impl_58 (impl)
macro_rules! Depcrateimpl_58 {
() => {
// Module: crate
// Provides: {"impl_58"}
// Dependencies: {}
# [cfg (feature = "malloc_size_of")] impl < T : MallocSizeOf > MallocSizeOf for ThinVec < T > { fn size_of (& self , ops : & mut MallocSizeOfOps) -> usize { let mut n = self . shallow_size_of (ops) ; for elem in self . iter () { n += elem . size_of (ops) ; } n } }
};
}
