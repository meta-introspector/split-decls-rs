// Generated macro for impl_415 (impl)
macro_rules! Depcrate_varzerovec_vecimpl_415 {
() => {
// Module: crate::varzerovec::vec
// Provides: {"impl_415"}
// Dependencies: {}
impl < 'a , T : ? Sized , F > From < & 'a VarZeroSlice < T , F > > for VarZeroVec < 'a , T , F > { fn from (other : & 'a VarZeroSlice < T , F >) -> Self { Self (VarZeroVecInner :: Borrowed (other)) } }
};
}
