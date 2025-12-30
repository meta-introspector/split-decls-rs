// Generated macro for impl_65 (impl)
macro_rules! Depcrate_sliceimpl_65 {
() => {
// Module: crate::slice
// Provides: {"impl_65"}
// Dependencies: {}
# [cfg (all (feature = "alloc" , feature = "zeroize"))] impl < const UPPERCASE : bool > Zeroize for HexOrBin < UPPERCASE > { fn zeroize (& mut self) { self . 0 . as_mut_slice () . zeroize () ; } }
};
}
