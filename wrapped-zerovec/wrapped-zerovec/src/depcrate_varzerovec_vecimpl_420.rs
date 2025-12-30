// Generated macro for impl_420 (impl)
macro_rules! Depcrate_varzerovec_vecimpl_420 {
() => {
// Module: crate::varzerovec::vec
// Provides: {"impl_420"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < A , T , F > From < & alloc :: vec :: Vec < A > > for VarZeroVec < 'static , T , F > where T : VarULE + ? Sized , A : EncodeAsVarULE < T > , F : VarZeroVecFormat , { # [inline] fn from (elements : & alloc :: vec :: Vec < A >) -> Self { Self :: from (elements . as_slice ()) } }
};
}
