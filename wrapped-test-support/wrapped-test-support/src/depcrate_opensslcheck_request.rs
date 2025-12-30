// Generated macro for check_request (function)
macro_rules! Depcrate_opensslcheck_request {
() => {
// Module: crate::openssl
// Provides: {"check_request"}
// Dependencies: {}
pub fn check_request (pem : & [u8]) -> String { check_openssl_output (& ["req" , "-verify"] , pem) }
};
}
