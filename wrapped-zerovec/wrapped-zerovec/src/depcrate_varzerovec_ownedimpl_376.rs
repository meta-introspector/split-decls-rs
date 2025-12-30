// Generated macro for impl_376 (impl)
macro_rules! Depcrate_varzerovec_ownedimpl_376 {
() => {
// Module: crate::varzerovec::owned
// Provides: {"impl_376"}
// Dependencies: {}
impl < T : VarULE + ? Sized , F : VarZeroVecFormat > fmt :: Debug for VarZeroVecOwned < T , F > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { VarZeroSlice :: fmt (self , f) } }
};
}
