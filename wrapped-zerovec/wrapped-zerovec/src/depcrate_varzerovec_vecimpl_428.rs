// Generated macro for impl_428 (impl)
macro_rules! Depcrate_varzerovec_vecimpl_428 {
() => {
// Module: crate::varzerovec::vec
// Provides: {"impl_428"}
// Dependencies: {}
impl < 'a , T : VarULE + ? Sized + Ord , F : VarZeroVecFormat > Ord for VarZeroVec < 'a , T , F > { fn cmp (& self , other : & Self) -> Ordering { self . iter () . cmp (other . iter ()) } }
};
}
