// Generated macro for impl_600 (impl)
macro_rules! Depcrate_ule_encodeimpl_600 {
() => {
// Module: crate::ule::encode
// Provides: {"impl_600"}
// Dependencies: {}
# [cfg (feature = "alloc")] unsafe impl < T > EncodeAsVarULE < [T] > for Vec < T > where T : ULE , { fn encode_var_ule_as_slices < R > (& self , cb : impl FnOnce (& [& [u8]]) -> R) -> R { cb (& [< [T] as VarULE > :: as_bytes (self)]) } }
};
}
