// Generated macro for impl_426 (impl)
macro_rules! Depcrate_varzerovec_vecimpl_426 {
() => {
// Module: crate::varzerovec::vec
// Provides: {"impl_426"}
// Dependencies: {}
impl < T , A , F , const N : usize > PartialEq < [A ; N] > for VarZeroVec < '_ , T , F > where T : VarULE + ? Sized , T : PartialEq , A : AsRef < T > , F : VarZeroVecFormat , { # [inline] fn eq (& self , other : & [A ; N]) -> bool { self . iter () . eq (other . iter () . map (| t | t . as_ref ())) } }
};
}
