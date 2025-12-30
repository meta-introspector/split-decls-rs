// Generated macro for impl_21 (impl)
macro_rules! Depcrate_arrayvecimpl_21 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_21"}
// Dependencies: {}
impl < A : Array , I : SliceIndex < [A :: Item] > > Index < I > for ArrayVec < A > { type Output = < I as SliceIndex < [A :: Item] > > :: Output ; # [inline (always)] fn index (& self , index : I) -> & Self :: Output { & self . deref () [index] } }
};
}
