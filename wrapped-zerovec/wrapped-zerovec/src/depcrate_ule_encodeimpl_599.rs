// Generated macro for impl_599 (impl)
macro_rules! Depcrate_ule_encodeimpl_599 {
() => {
// Module: crate::ule::encode
// Provides: {"impl_599"}
// Dependencies: {}
# [cfg (feature = "alloc")] unsafe impl EncodeAsVarULE < str > for & '_ String { fn encode_var_ule_as_slices < R > (& self , cb : impl FnOnce (& [& [u8]]) -> R) -> R { cb (& [self . as_bytes ()]) } }
};
}
