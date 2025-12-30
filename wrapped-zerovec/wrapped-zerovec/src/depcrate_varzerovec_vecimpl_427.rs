// Generated macro for impl_427 (impl)
macro_rules! Depcrate_varzerovec_vecimpl_427 {
() => {
// Module: crate::varzerovec::vec
// Provides: {"impl_427"}
// Dependencies: {}
impl < 'a , T : VarULE + ? Sized + PartialOrd , F : VarZeroVecFormat > PartialOrd for VarZeroVec < 'a , T , F > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . iter () . partial_cmp (other . iter ()) } }
};
}
