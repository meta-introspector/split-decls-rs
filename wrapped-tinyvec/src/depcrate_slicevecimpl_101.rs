// Generated macro for impl_101 (impl)
macro_rules! Depcrate_slicevecimpl_101 {
() => {
// Module: crate::slicevec
// Provides: {"impl_101"}
// Dependencies: {}
impl < 's , T , I > IndexMut < I > for SliceVec < 's , T > where I : SliceIndex < [T] > , { # [inline (always)] fn index_mut (& mut self , index : I) -> & mut Self :: Output { & mut self . deref_mut () [index] } }
};
}
