// Generated macro for impl_593 (impl)
macro_rules! Depcrate_ule_encodeimpl_593 {
() => {
// Module: crate::ule::encode
// Provides: {"impl_593"}
// Dependencies: {}
unsafe impl < T : VarULE + ? Sized > EncodeAsVarULE < T > for & '_ T { fn encode_var_ule_as_slices < R > (& self , cb : impl FnOnce (& [& [u8]]) -> R) -> R { cb (& [T :: as_bytes (self)]) } }
};
}
