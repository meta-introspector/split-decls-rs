// Generated macro for compute_serializable_len_without_length (function)
macro_rules! Depcrate_varzerovec_componentscompute_serializable_len_without_length {
() => {
// Module: crate::varzerovec::components
// Provides: {"compute_serializable_len_without_length"}
// Dependencies: {}
pub fn compute_serializable_len_without_length < T , A , F > (elements : & [A]) -> Option < u32 > where T : VarULE + ? Sized , A : EncodeAsVarULE < T > , F : VarZeroVecFormat , { let elements_len = elements . len () ; let Some (elements_len_minus_one) = elements_len . checked_sub (1) else { return Some (0) ; } ; let idx_len : u32 = u32 :: try_from (elements_len_minus_one) . ok () ? . checked_mul (F :: Index :: SIZE as u32) ? ; let data_len : u32 = elements . iter () . map (| v | u32 :: try_from (v . encode_var_ule_len ()) . ok ()) . try_fold (0u32 , | s , v | s . checked_add (v ?)) ? ; let ret = idx_len . checked_add (data_len) ; if let Some (r) = ret { if r >= F :: Index :: MAX_VALUE { return None ; } } ret }
};
}
