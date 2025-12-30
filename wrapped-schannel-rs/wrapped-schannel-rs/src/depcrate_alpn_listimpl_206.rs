// Generated macro for impl_206 (impl)
macro_rules! Depcrate_alpn_listimpl_206 {
() => {
// Module: crate::alpn_list
// Provides: {"impl_206"}
// Dependencies: {}
impl std :: ops :: DerefMut for AlpnList { fn deref_mut (& mut self) -> & mut Self :: Target { unsafe { slice :: from_raw_parts_mut (self . memory . as_ptr () , self . layout . size ()) } } }
};
}
