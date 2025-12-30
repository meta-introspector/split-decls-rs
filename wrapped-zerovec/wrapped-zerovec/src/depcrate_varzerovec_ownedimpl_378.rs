// Generated macro for impl_378 (impl)
macro_rules! Depcrate_varzerovec_ownedimpl_378 {
() => {
// Module: crate::varzerovec::owned
// Provides: {"impl_378"}
// Dependencies: {}
impl < T , A , F > PartialEq < & '_ [A] > for VarZeroVecOwned < T , F > where T : VarULE + ? Sized , T : PartialEq , A : AsRef < T > , F : VarZeroVecFormat , { # [inline] fn eq (& self , other : & & [A]) -> bool { self . iter () . eq (other . iter () . map (| t | t . as_ref ())) } }
};
}
