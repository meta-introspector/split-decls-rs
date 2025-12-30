// Generated macro for impl_595 (impl)
macro_rules! Depcrate_ule_encodeimpl_595 {
() => {
// Module: crate::ule::encode
// Provides: {"impl_595"}
// Dependencies: {}
# [cfg (feature = "alloc")] unsafe impl < T : VarULE + ? Sized > EncodeAsVarULE < T > for Cow < '_ , T > where T : ToOwned , { fn encode_var_ule_as_slices < R > (& self , cb : impl FnOnce (& [& [u8]]) -> R) -> R { cb (& [T :: as_bytes (self . as_ref ())]) } }
};
}
