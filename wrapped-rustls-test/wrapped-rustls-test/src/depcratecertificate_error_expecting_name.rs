// Generated macro for certificate_error_expecting_name (function)
macro_rules! Depcratecertificate_error_expecting_name {
() => {
// Module: crate
// Provides: {"certificate_error_expecting_name"}
// Dependencies: {}
pub fn certificate_error_expecting_name (expected : & str) -> CertificateError { CertificateError :: NotValidForNameContext { expected : ServerName :: try_from (expected) . unwrap () . to_owned () , presented : vec ! [r#"DnsName("testserver.com")"# . into () , r#"DnsName("second.testserver.com")"# . into () , r#"DnsName("localhost")"# . into () , "IpAddress(198.51.100.1)" . into () , "IpAddress(2001:db8::1)" . into () ,] , } }
};
}
