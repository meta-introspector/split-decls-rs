// Generated macro for check_crl (function)
macro_rules! Depcrate_opensslcheck_crl {
() => {
// Module: crate::openssl
// Provides: {"check_crl"}
// Dependencies: {}
pub fn check_crl (pem : & [u8]) -> String { check_openssl_output (& ["crl"] , pem) }
};
}
