// Generated macro for impl_759 (impl)
macro_rules! Depcrate_ule_vartupleimpl_759 {
() => {
// Module: crate::ule::vartuple
// Provides: {"impl_759"}
// Dependencies: {}
unsafe impl < A , B , V > EncodeAsVarULE < VarTupleULE < A , V > > for VarTuple < A , B > where A : AsULE + 'static , B : EncodeAsVarULE < V > , V : VarULE + ? Sized , { fn encode_var_ule_as_slices < R > (& self , _ : impl FnOnce (& [& [u8]]) -> R) -> R { unreachable ! () } # [inline] fn encode_var_ule_len (& self) -> usize { size_of :: < A :: ULE > () + self . variable . encode_var_ule_len () } # [inline] fn encode_var_ule_write (& self , dst : & mut [u8]) { let (sized_chunk , variable_chunk) = dst . split_at_mut (size_of :: < A :: ULE > ()) ; sized_chunk . clone_from_slice ([self . sized . to_unaligned ()] . as_bytes ()) ; self . variable . encode_var_ule_write (variable_chunk) ; } }
};
}
