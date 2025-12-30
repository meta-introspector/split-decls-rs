// Generated macro for impl_193 (impl)
macro_rules! Depcrate_tinyvecimpl_193 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_193"}
// Dependencies: {}
impl < A : Array > PartialEq < & A > for TinyVec < A > where A :: Item : PartialEq , { # [inline] fn eq (& self , other : & & A) -> bool { self . as_slice () . eq (other . as_slice ()) } }
};
}
