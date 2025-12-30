// Generated macro for impl_550 (impl)
macro_rules! Depcrate_zerovecimpl_550 {
() => {
// Module: crate::zerovec
// Provides: {"impl_550"}
// Dependencies: {}
impl < T > PartialEq < & [T] > for ZeroVec < '_ , T > where T : AsULE + PartialEq , { # [inline] fn eq (& self , other : & & [T]) -> bool { self . iter () . eq (other . iter () . copied ()) } }
};
}
