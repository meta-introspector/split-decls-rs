// Generated macro for SSLWriteFunc (type)
macro_rules! Depcrate_secure_transportSSLWriteFunc {
() => {
// Module: crate::secure_transport
// Provides: {"SSLWriteFunc"}
// Dependencies: {}
pub type SSLWriteFunc = unsafe extern "C" fn (connection : SSLConnectionRef , data : * const c_void , dataLength : * mut usize ,) -> OSStatus ;
};
}
