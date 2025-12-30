// Generated macro for impl_425 (impl)
macro_rules! Depcrate_varzerovec_vecimpl_425 {
() => {
// Module: crate::varzerovec::vec
// Provides: {"impl_425"}
// Dependencies: {}
impl < T , A , F > PartialEq < & '_ [A] > for VarZeroVec < '_ , T , F > where T : VarULE + ? Sized , T : PartialEq , A : AsRef < T > , F : VarZeroVecFormat , { # [inline] fn eq (& self , other : & & [A]) -> bool { self . iter () . eq (other . iter () . map (| t | t . as_ref ())) } }
};
}
