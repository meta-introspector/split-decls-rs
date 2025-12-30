// Generated macro for impl_109 (impl)
macro_rules! Depcrate_slicevecimpl_109 {
() => {
// Module: crate::slicevec
// Provides: {"impl_109"}
// Dependencies: {}
impl < 'p , 's , T : Default > Drop for SliceVecDrain < 'p , 's , T > { # [inline] fn drop (& mut self) { self . for_each (drop) ; let count = self . target_end - self . target_start ; let targets : & mut [T] = & mut self . parent . deref_mut () [self . target_start ..] ; targets . rotate_left (count) ; self . parent . len -= count ; } }
};
}
