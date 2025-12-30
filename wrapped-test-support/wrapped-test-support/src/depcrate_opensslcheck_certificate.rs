// Generated macro for check_certificate (function)
macro_rules! Depcrate_opensslcheck_certificate {
() => {
// Module: crate::openssl
// Provides: {"check_certificate"}
// Dependencies: {}
pub fn check_certificate (pem : & [u8]) -> String { check_openssl_output (& ["x509"] , pem) }
};
}
