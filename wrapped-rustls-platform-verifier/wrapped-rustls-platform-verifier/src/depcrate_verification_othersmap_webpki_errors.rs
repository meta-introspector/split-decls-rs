// Generated macro for map_webpki_errors (function)
macro_rules! Depcrate_verification_othersmap_webpki_errors {
() => {
// Module: crate::verification::others
// Provides: {"map_webpki_errors"}
// Dependencies: {}
fn map_webpki_errors (err : TlsError) -> TlsError { match & err { TlsError :: InvalidCertificate (CertificateError :: InvalidPurpose) | TlsError :: InvalidCertificate (CertificateError :: InvalidPurposeContext { .. }) => { TlsError :: InvalidCertificate (CertificateError :: Other (OtherError (Arc :: new (super :: EkuError ,)))) } _ => err , } }
};
}
