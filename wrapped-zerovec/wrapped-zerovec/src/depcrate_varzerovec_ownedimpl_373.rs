// Generated macro for impl_373 (impl)
macro_rules! Depcrate_varzerovec_ownedimpl_373 {
() => {
// Module: crate::varzerovec::owned
// Provides: {"impl_373"}
// Dependencies: {}
impl < T : VarULE + ? Sized , F : VarZeroVecFormat > Deref for VarZeroVecOwned < T , F > { type Target = VarZeroSlice < T , F > ; fn deref (& self) -> & VarZeroSlice < T , F > { self . as_slice () } }
};
}
