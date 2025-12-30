// Generated macro for impl_422 (impl)
macro_rules! Depcrate_varzerovec_vecimpl_422 {
() => {
// Module: crate::varzerovec::vec
// Provides: {"impl_422"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < A , T , F , const N : usize > From < & [A ; N] > for VarZeroVec < 'static , T , F > where T : VarULE + ? Sized , A : EncodeAsVarULE < T > , F : VarZeroVecFormat , { # [inline] fn from (elements : & [A ; N]) -> Self { Self :: from (elements . as_slice ()) } }
};
}
