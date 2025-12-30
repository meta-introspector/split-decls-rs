// Generated macro for impl_421 (impl)
macro_rules! Depcrate_varzerovec_vecimpl_421 {
() => {
// Module: crate::varzerovec::vec
// Provides: {"impl_421"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < A , T , F > From < & [A] > for VarZeroVec < 'static , T , F > where T : VarULE + ? Sized , A : EncodeAsVarULE < T > , F : VarZeroVecFormat , { # [inline] fn from (elements : & [A]) -> Self { if elements . is_empty () { VarZeroSlice :: new_empty () . into () } else { # [expect (clippy :: unwrap_used)] VarZeroVecOwned :: try_from_elements (elements) . unwrap () . into () } } }
};
}
