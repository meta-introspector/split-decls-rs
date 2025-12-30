// Generated macro for impl_205 (impl)
macro_rules! Depcrate_alpn_listimpl_205 {
() => {
// Module: crate::alpn_list
// Provides: {"impl_205"}
// Dependencies: {}
impl std :: ops :: Deref for AlpnList { type Target = [u8] ; fn deref (& self) -> & Self :: Target { unsafe { slice :: from_raw_parts (self . memory . as_ptr () , self . layout . size ()) } } }
};
}
