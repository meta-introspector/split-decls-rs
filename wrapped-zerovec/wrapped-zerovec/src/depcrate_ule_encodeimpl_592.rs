// Generated macro for impl_592 (impl)
macro_rules! Depcrate_ule_encodeimpl_592 {
() => {
// Module: crate::ule::encode
// Provides: {"impl_592"}
// Dependencies: {}
unsafe impl < T : VarULE + ? Sized > EncodeAsVarULE < T > for T { fn encode_var_ule_as_slices < R > (& self , cb : impl FnOnce (& [& [u8]]) -> R) -> R { cb (& [T :: as_bytes (self)]) } }
};
}
