// Generated macro for impl_64 (impl)
macro_rules! Depcrate_arrayvecimpl_64 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_64"}
// Dependencies: {}
impl < A : Array > PartialEq < & A > for ArrayVec < A > where A :: Item : PartialEq , { # [inline] fn eq (& self , other : & & A) -> bool { self . as_slice () . eq (other . as_slice ()) } }
};
}
