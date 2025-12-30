// Generated macro for impl_103 (impl)
macro_rules! Depcrate_arc_unionimpl_103 {
() => {
// Module: crate::arc_union
// Provides: {"impl_103"}
// Dependencies: {}
impl < A : PartialEq , B : PartialEq > PartialEq for ArcUnion < A , B > { fn eq (& self , other : & Self) -> bool { use crate :: ArcUnionBorrow :: * ; match (self . borrow () , other . borrow ()) { (First (x) , First (y)) => x == y , (Second (x) , Second (y)) => x == y , (_ , _) => false , } } }
};
}
