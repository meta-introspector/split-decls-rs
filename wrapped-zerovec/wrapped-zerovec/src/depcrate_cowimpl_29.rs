// Generated macro for impl_29 (impl)
macro_rules! Depcrate_cowimpl_29 {
() => {
// Module: crate::cow
// Provides: {"impl_29"}
// Dependencies: {}
unsafe impl < 'a , V : VarULE + ? Sized > EncodeAsVarULE < V > for VarZeroCow < 'a , V > { fn encode_var_ule_as_slices < R > (& self , _ : impl FnOnce (& [& [u8]]) -> R) -> R { unreachable ! () } # [inline] fn encode_var_ule_len (& self) -> usize { self . as_bytes () . len () } # [inline] fn encode_var_ule_write (& self , dst : & mut [u8]) { dst . copy_from_slice (self . as_bytes ()) } }
};
}
