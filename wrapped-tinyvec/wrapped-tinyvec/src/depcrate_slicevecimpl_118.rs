// Generated macro for impl_118 (impl)
macro_rules! Depcrate_slicevecimpl_118 {
() => {
// Module: crate::slicevec
// Provides: {"impl_118"}
// Dependencies: {}
impl < 's , T > PartialOrd for SliceVec < 's , T > where T : PartialOrd , { # [inline] fn partial_cmp (& self , other : & Self) -> Option < core :: cmp :: Ordering > { self . as_slice () . partial_cmp (other . as_slice ()) } }
};
}
