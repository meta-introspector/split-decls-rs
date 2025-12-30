// Generated macro for impl_596 (impl)
macro_rules! Depcrate_ule_encodeimpl_596 {
() => {
// Module: crate::ule::encode
// Provides: {"impl_596"}
// Dependencies: {}
# [cfg (feature = "alloc")] unsafe impl < T : VarULE + ? Sized > EncodeAsVarULE < T > for Box < T > { fn encode_var_ule_as_slices < R > (& self , cb : impl FnOnce (& [& [u8]]) -> R) -> R { cb (& [T :: as_bytes (self)]) } }
};
}
