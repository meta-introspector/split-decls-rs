// Generated macro for impl_107 (impl)
macro_rules! Depcrate_arc_unionimpl_107 {
() => {
// Module: crate::arc_union
// Provides: {"impl_107"}
// Dependencies: {}
impl < A , B > Drop for ArcUnion < A , B > { fn drop (& mut self) { match self . borrow () { ArcUnionBorrow :: First (x) => unsafe { let _ = Arc :: from_raw (& * x) ; } , ArcUnionBorrow :: Second (x) => unsafe { let _ = Arc :: from_raw (& * x) ; } , } } }
};
}
