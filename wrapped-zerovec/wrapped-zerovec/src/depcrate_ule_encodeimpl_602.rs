// Generated macro for impl_602 (impl)
macro_rules! Depcrate_ule_encodeimpl_602 {
() => {
// Module: crate::ule::encode
// Provides: {"impl_602"}
// Dependencies: {}
# [cfg (feature = "alloc")] unsafe impl < T > EncodeAsVarULE < ZeroSlice < T > > for Vec < T > where T : AsULE + 'static , { fn encode_var_ule_as_slices < R > (& self , _ : impl FnOnce (& [& [u8]]) -> R) -> R { unreachable ! () } # [inline] fn encode_var_ule_len (& self) -> usize { self . as_slice () . encode_var_ule_len () } # [inline] fn encode_var_ule_write (& self , dst : & mut [u8]) { self . as_slice () . encode_var_ule_write (dst) } }
};
}
