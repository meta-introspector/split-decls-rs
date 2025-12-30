// Generated macro for impl_100 (impl)
macro_rules! Depcrate_slicevecimpl_100 {
() => {
// Module: crate::slicevec
// Provides: {"impl_100"}
// Dependencies: {}
impl < 's , T , I > Index < I > for SliceVec < 's , T > where I : SliceIndex < [T] > , { type Output = < I as SliceIndex < [T] > > :: Output ; # [inline (always)] fn index (& self , index : I) -> & Self :: Output { & self . deref () [index] } }
};
}
