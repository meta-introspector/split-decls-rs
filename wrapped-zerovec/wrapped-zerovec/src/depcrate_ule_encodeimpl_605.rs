// Generated macro for impl_605 (impl)
macro_rules! Depcrate_ule_encodeimpl_605 {
() => {
// Module: crate::ule::encode
// Provides: {"impl_605"}
// Dependencies: {}
# [cfg (feature = "alloc")] unsafe impl < T , E , F > EncodeAsVarULE < VarZeroSlice < T , F > > for Vec < E > where T : VarULE + ? Sized , E : EncodeAsVarULE < T > , F : VarZeroVecFormat , { fn encode_var_ule_as_slices < R > (& self , _ : impl FnOnce (& [& [u8]]) -> R) -> R { unreachable ! () } # [inline] fn encode_var_ule_len (& self) -> usize { < _ as EncodeAsVarULE < VarZeroSlice < T , F > > > :: encode_var_ule_len (& self . as_slice ()) } # [inline] fn encode_var_ule_write (& self , dst : & mut [u8]) { < _ as EncodeAsVarULE < VarZeroSlice < T , F > > > :: encode_var_ule_write (& self . as_slice () , dst) } }
};
}
