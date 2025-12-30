// Generated macro for impl_413 (impl)
macro_rules! Depcrate_varzerovec_vecimpl_413 {
() => {
// Module: crate::varzerovec::vec
// Provides: {"impl_413"}
// Dependencies: {}
impl < T : VarULE + ? Sized , F : VarZeroVecFormat > fmt :: Debug for VarZeroVec < '_ , T , F > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { VarZeroSlice :: fmt (self , f) } }
};
}
