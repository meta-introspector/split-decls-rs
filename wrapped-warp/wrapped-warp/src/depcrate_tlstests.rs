// Generated macro for tests (module)
macro_rules! Depcrate_tlstests {
() => {
// Module: crate::tls
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn file_cert_key () { TlsConfigBuilder :: new () . key_path ("examples/tls/key.rsa") . cert_path ("examples/tls/cert.pem") . build () . unwrap () ; } # [test] fn bytes_cert_key () { let key = include_str ! ("../examples/tls/key.rsa") ; let cert = include_str ! ("../examples/tls/cert.pem") ; TlsConfigBuilder :: new () . key (key . as_bytes ()) . cert (cert . as_bytes ()) . build () . unwrap () ; } # [test] fn file_ecc_cert_key () { TlsConfigBuilder :: new () . key_path ("examples/tls/key.ecc") . cert_path ("examples/tls/cert.ecc.pem") . build () . unwrap () ; } # [test] fn bytes_ecc_cert_key () { let key = include_str ! ("../examples/tls/key.ecc") ; let cert = include_str ! ("../examples/tls/cert.ecc.pem") ; TlsConfigBuilder :: new () . key (key . as_bytes ()) . cert (cert . as_bytes ()) . build () . unwrap () ; } }
};
}
