// Generated macro for CertificateKind (enum)
macro_rules! Depcrate_settings_tlsCertificateKind {
() => {
// Module: crate::settings::tls
// Provides: {"CertificateKind"}
// Dependencies: {}
# [doc = " Types of PKI certificates supported by the crate."] # [derive (Default , Clone , Copy , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] pub enum CertificateKind { # [doc = " Standard X509 TLS certificate."] # [default] X509 , # [doc = " [Raw public key] TLS certificate."] # [doc = ""] # [doc = ""] # [doc = " [Raw public key]: https://datatracker.ietf.org/doc/html/rfc7250"] RawPublicKey , }
};
}
