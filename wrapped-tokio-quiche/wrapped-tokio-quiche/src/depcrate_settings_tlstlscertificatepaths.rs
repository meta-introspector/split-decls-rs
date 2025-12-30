// Generated macro for TlsCertificatePaths (struct)
macro_rules! Depcrate_settings_tlsTlsCertificatePaths {
() => {
// Module: crate::settings::tls
// Provides: {"TlsCertificatePaths"}
// Dependencies: {}
# [doc = " TLS credentials to authenticate the endpoint."] # [derive (Clone , Copy , Debug)] pub struct TlsCertificatePaths < 'p > { # [doc = " Path to the endpoint's TLS certificate."] pub cert : & 'p str , # [doc = " Path to the endpoint's private key."] pub private_key : & 'p str , # [doc = " `cert`'s PKI certificate type."] pub kind : CertificateKind , }
};
}
