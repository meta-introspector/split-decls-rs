// Generated macro for impl_598 (impl)
macro_rules! Depcrate_ule_encodeimpl_598 {
() => {
// Module: crate::ule::encode
// Provides: {"impl_598"}
// Dependencies: {}
# [cfg (feature = "alloc")] unsafe impl EncodeAsVarULE < str > for String { fn encode_var_ule_as_slices < R > (& self , cb : impl FnOnce (& [& [u8]]) -> R) -> R { cb (& [self . as_bytes ()]) } }
};
}
