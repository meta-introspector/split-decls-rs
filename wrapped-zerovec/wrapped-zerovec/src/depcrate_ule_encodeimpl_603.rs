// Generated macro for impl_603 (impl)
macro_rules! Depcrate_ule_encodeimpl_603 {
() => {
// Module: crate::ule::encode
// Provides: {"impl_603"}
// Dependencies: {}
unsafe impl < T > EncodeAsVarULE < ZeroSlice < T > > for ZeroVec < '_ , T > where T : AsULE + 'static , { fn encode_var_ule_as_slices < R > (& self , _ : impl FnOnce (& [& [u8]]) -> R) -> R { unreachable ! () } # [inline] fn encode_var_ule_len (& self) -> usize { self . as_bytes () . len () } fn encode_var_ule_write (& self , dst : & mut [u8]) { debug_assert_eq ! (self . as_bytes () . len () , dst . len ()) ; dst . copy_from_slice (self . as_bytes ()) ; } }
};
}
