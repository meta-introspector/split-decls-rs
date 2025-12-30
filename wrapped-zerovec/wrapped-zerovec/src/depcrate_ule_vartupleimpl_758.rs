// Generated macro for impl_758 (impl)
macro_rules! Depcrate_ule_vartupleimpl_758 {
() => {
// Module: crate::ule::vartuple
// Provides: {"impl_758"}
// Dependencies: {}
unsafe impl < A , V > VarULE for VarTupleULE < A , V > where A : AsULE + 'static , V : VarULE + ? Sized , { fn validate_bytes (bytes : & [u8]) -> Result < () , UleError > { let (sized_chunk , variable_chunk) = bytes . split_at_checked (size_of :: < A :: ULE > ()) . ok_or (UleError :: length :: < Self > (bytes . len ())) ? ; A :: ULE :: validate_bytes (sized_chunk) ? ; V :: validate_bytes (variable_chunk) ? ; Ok (()) } unsafe fn from_bytes_unchecked (bytes : & [u8]) -> & Self { let (_sized_chunk , variable_chunk) = bytes . split_at_unchecked (size_of :: < A :: ULE > ()) ; let variable_ref = V :: from_bytes_unchecked (variable_chunk) ; let variable_ptr : * const V = variable_ref ; assert_eq ! (size_of ::<* const V > () , size_of ::< (* const u8 , usize) > ()) ; let (_v_ptr , metadata) = transmute_copy :: < * const V , (* const u8 , usize) > (& variable_ptr) ; assert_eq ! (size_of ::<* const Self > () , size_of ::< (* const u8 , usize) > ()) ; let composed_ptr = transmute_copy :: < (* const u8 , usize) , * const Self > (& (bytes . as_ptr () , metadata)) ; & * (composed_ptr) } }
};
}
