// Generated macro for impl_324 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_324 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_324"}
// Dependencies: {}
impl < 'a > CertificatePayloadTls13 < 'a > { pub (crate) fn new (certs : impl Iterator < Item = CertificateDer < 'a > > , ocsp_response : Option < & 'a [u8] > ,) -> Self { let ocsp_response = match ocsp_response { Some ([]) | None => None , Some (bytes) => Some (bytes) , } ; Self { context : PayloadU8 :: empty () , entries : certs . zip (ocsp_response . into_iter () . map (Some) . chain (iter :: repeat (None)) ,) . map (| (cert , ocsp) | { let mut e = CertificateEntry :: new (cert . clone ()) ; if let Some (ocsp) = ocsp { e . extensions . status = Some (CertificateStatus :: new (ocsp)) ; } e }) . collect () , } } pub (crate) fn into_owned (self) -> CertificatePayloadTls13 < 'static > { CertificatePayloadTls13 { context : self . context , entries : self . entries . into_iter () . map (CertificateEntry :: into_owned) . collect () , } } pub (crate) fn end_entity_ocsp (& self) -> Vec < u8 > { let Some (entry) = self . entries . first () else { return vec ! [] ; } ; entry . extensions . status . as_ref () . map (| status | status . ocsp_response . as_ref () . to_vec ()) . unwrap_or_default () } pub (crate) fn into_certificate_chain (self) -> CertificateChain < 'a > { CertificateChain (self . entries . into_iter () . map (| e | e . cert) . collect () ,) } }
};
}
