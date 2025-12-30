// Generated macro for VarZeroVecInner (enum)
macro_rules! Depcrate_varzerovec_vecVarZeroVecInner {
() => {
// Module: crate::varzerovec::vec
// Provides: {"VarZeroVecInner"}
// Dependencies: {}
pub (crate) enum VarZeroVecInner < 'a , T : ? Sized , F = Index16 > { # [cfg (feature = "alloc")] Owned (VarZeroVecOwned < T , F >) , Borrowed (& 'a VarZeroSlice < T , F >) , }
};
}
