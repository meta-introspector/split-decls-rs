// Generated macro for impl_119 (impl)
macro_rules! Depcrate_slicevecimpl_119 {
() => {
// Module: crate::slicevec
// Provides: {"impl_119"}
// Dependencies: {}
impl < 's , T > Ord for SliceVec < 's , T > where T : Ord , { # [inline] fn cmp (& self , other : & Self) -> core :: cmp :: Ordering { self . as_slice () . cmp (other . as_slice ()) } }
};
}
