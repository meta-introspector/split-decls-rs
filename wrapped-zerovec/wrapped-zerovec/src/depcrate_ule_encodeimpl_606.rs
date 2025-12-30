// Generated macro for impl_606 (impl)
macro_rules! Depcrate_ule_encodeimpl_606 {
() => {
// Module: crate::ule::encode
// Provides: {"impl_606"}
// Dependencies: {}
unsafe impl < T , F > EncodeAsVarULE < VarZeroSlice < T , F > > for VarZeroVec < '_ , T , F > where T : VarULE + ? Sized , F : VarZeroVecFormat , { fn encode_var_ule_as_slices < R > (& self , _ : impl FnOnce (& [& [u8]]) -> R) -> R { unreachable ! () } # [inline] fn encode_var_ule_len (& self) -> usize { self . as_bytes () . len () } # [inline] fn encode_var_ule_write (& self , dst : & mut [u8]) { debug_assert_eq ! (self . as_bytes () . len () , dst . len ()) ; dst . copy_from_slice (self . as_bytes ()) ; } }
};
}
