// Generated macro for impl_189 (impl)
macro_rules! Depcrate_tinyvecimpl_189 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_189"}
// Dependencies: {}
impl < A : Array > PartialEq for TinyVec < A > where A :: Item : PartialEq , { # [inline] fn eq (& self , other : & Self) -> bool { self . as_slice () . eq (other . as_slice ()) } }
};
}
