// Generated macro for impl_23 (impl)
macro_rules! Depcrate_arrayimpl_23 {
() => {
// Module: crate::array
// Provides: {"impl_23"}
// Dependencies: {}
# [cfg (feature = "zeroize")] impl < const N : usize , const UPPERCASE : bool > Zeroize for HexOrBin < N , UPPERCASE > { fn zeroize (& mut self) { self . 0 . as_mut_slice () . zeroize () ; } }
};
}
