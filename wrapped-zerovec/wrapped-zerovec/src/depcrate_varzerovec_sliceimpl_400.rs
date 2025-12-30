// Generated macro for impl_400 (impl)
macro_rules! Depcrate_varzerovec_sliceimpl_400 {
() => {
// Module: crate::varzerovec::slice
// Provides: {"impl_400"}
// Dependencies: {}
impl < T : VarULE + ? Sized + PartialOrd , F : VarZeroVecFormat > PartialOrd for VarZeroSlice < T , F > { # [inline] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . iter () . partial_cmp (other . iter ()) } }
};
}
