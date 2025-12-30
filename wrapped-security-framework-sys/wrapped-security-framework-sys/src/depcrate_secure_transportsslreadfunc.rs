// Generated macro for SSLReadFunc (type)
macro_rules! Depcrate_secure_transportSSLReadFunc {
() => {
// Module: crate::secure_transport
// Provides: {"SSLReadFunc"}
// Dependencies: {}
pub type SSLReadFunc = unsafe extern "C" fn (connection : SSLConnectionRef , data : * mut c_void , dataLength : * mut usize ,) -> OSStatus ;
};
}
