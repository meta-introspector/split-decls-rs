// Generated macro for impl_194 (impl)
macro_rules! Depcrate_tinyvecimpl_194 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_194"}
// Dependencies: {}
impl < A : Array > PartialEq < & [A :: Item] > for TinyVec < A > where A :: Item : PartialEq , { # [inline] fn eq (& self , other : & & [A :: Item]) -> bool { self . as_slice () . eq (* other) } }
};
}
