// Generated macro for impl_594 (impl)
macro_rules! Depcrate_ule_encodeimpl_594 {
() => {
// Module: crate::ule::encode
// Provides: {"impl_594"}
// Dependencies: {}
unsafe impl < T : VarULE + ? Sized > EncodeAsVarULE < T > for & '_ & '_ T { fn encode_var_ule_as_slices < R > (& self , cb : impl FnOnce (& [& [u8]]) -> R) -> R { cb (& [T :: as_bytes (self)]) } }
};
}
