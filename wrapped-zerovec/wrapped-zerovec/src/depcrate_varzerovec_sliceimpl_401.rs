// Generated macro for impl_401 (impl)
macro_rules! Depcrate_varzerovec_sliceimpl_401 {
() => {
// Module: crate::varzerovec::slice
// Provides: {"impl_401"}
// Dependencies: {}
impl < T : VarULE + ? Sized + Ord , F : VarZeroVecFormat > Ord for VarZeroSlice < T , F > { # [inline] fn cmp (& self , other : & Self) -> Ordering { self . iter () . cmp (other . iter ()) } }
};
}
