// Generated macro for impl_659 (impl)
macro_rules! Depcrate_secure_transportimpl_659 {
() => {
// Module: crate::secure_transport
// Provides: {"impl_659"}
// Dependencies: {}
impl < S > Drop for SslStream < S > { fn drop (& mut self) { unsafe { let mut conn = ptr :: null () ; let ret = SSLGetConnection (self . ctx . 0 , & mut conn) ; assert ! (ret == errSecSuccess) ; let _ = Box :: < Connection < S > > :: from_raw (conn as * mut _) ; } } }
};
}
