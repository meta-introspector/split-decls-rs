// Generated macro for impl_22 (impl)
macro_rules! Depcrate_arrayvecimpl_22 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_22"}
// Dependencies: {}
impl < A : Array , I : SliceIndex < [A :: Item] > > IndexMut < I > for ArrayVec < A > { # [inline (always)] fn index_mut (& mut self , index : I) -> & mut Self :: Output { & mut self . deref_mut () [index] } }
};
}
