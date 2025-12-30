// Generated macro for impl_551 (impl)
macro_rules! Depcrate_zerovecimpl_551 {
() => {
// Module: crate::zerovec
// Provides: {"impl_551"}
// Dependencies: {}
impl < T , const N : usize > PartialEq < [T ; N] > for ZeroVec < '_ , T > where T : AsULE + PartialEq , { # [inline] fn eq (& self , other : & [T ; N]) -> bool { self . iter () . eq (other . iter () . copied ()) } }
};
}
