// Generated macro for impl_108 (impl)
macro_rules! Depcrate_arc_unionimpl_108 {
() => {
// Module: crate::arc_union
// Provides: {"impl_108"}
// Dependencies: {}
impl < A : fmt :: Debug , B : fmt :: Debug > fmt :: Debug for ArcUnion < A , B > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Debug :: fmt (& self . borrow () , f) } }
};
}
