// Generated macro for impl_106 (impl)
macro_rules! Depcrate_arc_unionimpl_106 {
() => {
// Module: crate::arc_union
// Provides: {"impl_106"}
// Dependencies: {}
impl < A , B > Clone for ArcUnion < A , B > { fn clone (& self) -> Self { match self . borrow () { ArcUnionBorrow :: First (x) => ArcUnion :: from_first (x . clone_arc ()) , ArcUnionBorrow :: Second (x) => ArcUnion :: from_second (x . clone_arc ()) , } } }
};
}
