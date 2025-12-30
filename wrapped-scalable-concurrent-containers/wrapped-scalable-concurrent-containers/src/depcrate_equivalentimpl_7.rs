// Generated macro for impl_7 (impl)
macro_rules! Depcrate_equivalentimpl_7 {
() => {
// Module: crate::equivalent
// Provides: {"impl_7"}
// Dependencies: {}
impl < Q : ? Sized , K : ? Sized > Equivalent < K > for Q where Q : Eq , K : Borrow < Q > , { # [inline] fn equivalent (& self , key : & K) -> bool { PartialEq :: eq (self , key . borrow ()) } }
};
}
