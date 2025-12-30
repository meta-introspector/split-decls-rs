// Generated macro for impl_60 (impl)
macro_rules! Depcrate_arrayvecimpl_60 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_60"}
// Dependencies: {}
impl < A : Array > PartialEq for ArrayVec < A > where A :: Item : PartialEq , { # [inline] fn eq (& self , other : & Self) -> bool { self . as_slice () . eq (other . as_slice ()) } }
};
}
