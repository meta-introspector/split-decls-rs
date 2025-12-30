// Generated macro for impl_2058 (impl)
macro_rules! Depcrate_webpki_verifyimpl_2058 {
() => {
// Module: crate::webpki::verify
// Provides: {"impl_2058"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a CertificateDer < 'a > > for ParsedCertificate < 'a > { type Error = Error ; fn try_from (value : & 'a CertificateDer < 'a >) -> Result < Self , Self :: Error > { webpki :: EndEntityCert :: try_from (value) . map_err (pki_error) . map (ParsedCertificate) } }
};
}
