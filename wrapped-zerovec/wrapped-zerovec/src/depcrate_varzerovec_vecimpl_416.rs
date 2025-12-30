// Generated macro for impl_416 (impl)
macro_rules! Depcrate_varzerovec_vecimpl_416 {
() => {
// Module: crate::varzerovec::vec
// Provides: {"impl_416"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'a , T : ? Sized + VarULE , F : VarZeroVecFormat > From < VarZeroVec < 'a , T , F > > for VarZeroVecOwned < T , F > { # [inline] fn from (other : VarZeroVec < 'a , T , F >) -> Self { match other . 0 { VarZeroVecInner :: Owned (o) => o , VarZeroVecInner :: Borrowed (b) => b . into () , } } }
};
}
