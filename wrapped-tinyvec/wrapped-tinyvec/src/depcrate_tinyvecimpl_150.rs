// Generated macro for impl_150 (impl)
macro_rules! Depcrate_tinyvecimpl_150 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_150"}
// Dependencies: {}
impl < A : Array , I : SliceIndex < [A :: Item] > > IndexMut < I > for TinyVec < A > { # [inline (always)] fn index_mut (& mut self , index : I) -> & mut Self :: Output { & mut self . deref_mut () [index] } }
};
}
