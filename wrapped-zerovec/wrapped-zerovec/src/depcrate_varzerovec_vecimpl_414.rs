// Generated macro for impl_414 (impl)
macro_rules! Depcrate_varzerovec_vecimpl_414 {
() => {
// Module: crate::varzerovec::vec
// Provides: {"impl_414"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'a , T : ? Sized , F > From < VarZeroVecOwned < T , F > > for VarZeroVec < 'a , T , F > { # [inline] fn from (other : VarZeroVecOwned < T , F >) -> Self { Self (VarZeroVecInner :: Owned (other)) } }
};
}
