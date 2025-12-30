// Generated macro for impl_604 (impl)
macro_rules! Depcrate_ule_encodeimpl_604 {
() => {
// Module: crate::ule::encode
// Provides: {"impl_604"}
// Dependencies: {}
unsafe impl < T , E , F > EncodeAsVarULE < VarZeroSlice < T , F > > for & '_ [E] where T : VarULE + ? Sized , E : EncodeAsVarULE < T > , F : VarZeroVecFormat , { fn encode_var_ule_as_slices < R > (& self , _ : impl FnOnce (& [& [u8]]) -> R) -> R { unimplemented ! () } # [expect (clippy :: unwrap_used)] fn encode_var_ule_len (& self) -> usize { crate :: varzerovec :: components :: compute_serializable_len :: < T , E , F > (self) . unwrap () as usize } fn encode_var_ule_write (& self , dst : & mut [u8]) { crate :: varzerovec :: components :: write_serializable_bytes :: < T , E , F > (self , dst) } }
};
}
