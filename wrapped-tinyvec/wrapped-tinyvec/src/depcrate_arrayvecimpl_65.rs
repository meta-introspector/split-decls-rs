// Generated macro for impl_65 (impl)
macro_rules! Depcrate_arrayvecimpl_65 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_65"}
// Dependencies: {}
impl < A : Array > PartialEq < & [A :: Item] > for ArrayVec < A > where A :: Item : PartialEq , { # [inline] fn eq (& self , other : & & [A :: Item]) -> bool { self . as_slice () . eq (* other) } }
};
}
