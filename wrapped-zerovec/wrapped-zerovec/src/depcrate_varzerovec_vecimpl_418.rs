// Generated macro for impl_418 (impl)
macro_rules! Depcrate_varzerovec_vecimpl_418 {
() => {
// Module: crate::varzerovec::vec
// Provides: {"impl_418"}
// Dependencies: {}
impl < T : VarULE + ? Sized , F : VarZeroVecFormat > Deref for VarZeroVec < '_ , T , F > { type Target = VarZeroSlice < T , F > ; fn deref (& self) -> & VarZeroSlice < T , F > { self . as_slice () } }
};
}
