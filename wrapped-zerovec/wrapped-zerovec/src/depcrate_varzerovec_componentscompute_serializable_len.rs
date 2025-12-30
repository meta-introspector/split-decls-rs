// Generated macro for compute_serializable_len (function)
macro_rules! Depcrate_varzerovec_componentscompute_serializable_len {
() => {
// Module: crate::varzerovec::components
// Provides: {"compute_serializable_len"}
// Dependencies: {}
pub fn compute_serializable_len < T , A , F > (elements : & [A]) -> Option < u32 > where T : VarULE + ? Sized , A : EncodeAsVarULE < T > , F : VarZeroVecFormat , { compute_serializable_len_without_length :: < T , A , F > (elements) . map (| x | x + F :: Len :: SIZE as u32) }
};
}
