// Generated macro for load_native_certs (function)
macro_rules! Depcrate_unixload_native_certs {
() => {
// Module: crate::unix
// Provides: {"load_native_certs"}
// Dependencies: {}
pub fn load_native_certs () -> CertificateResult { let likely_locations = openssl_probe :: probe () ; CertPaths { file : likely_locations . cert_file , dirs : likely_locations . cert_dir . into_iter () . collect () , } . load () }
};
}
