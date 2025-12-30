// Generated macro for impl_412 (impl)
macro_rules! Depcrate_varzerovec_vecimpl_412 {
() => {
// Module: crate::varzerovec::vec
// Provides: {"impl_412"}
// Dependencies: {}
impl < 'a , T : ? Sized , F > Clone for VarZeroVec < 'a , T , F > { fn clone (& self) -> Self { match self . 0 { # [cfg (feature = "alloc")] VarZeroVecInner :: Owned (ref o) => o . clone () . into () , VarZeroVecInner :: Borrowed (b) => b . into () , } } }
};
}
