// Generated macro for impl_149 (impl)
macro_rules! Depcrate_tinyvecimpl_149 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_149"}
// Dependencies: {}
impl < A : Array , I : SliceIndex < [A :: Item] > > Index < I > for TinyVec < A > { type Output = < I as SliceIndex < [A :: Item] > > :: Output ; # [inline (always)] fn index (& self , index : I) -> & Self :: Output { & self . deref () [index] } }
};
}
