// Generated macro for impl_374 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_374 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_374"}
// Dependencies: {}
impl < 'a > CertificateStatus < 'a > { pub (crate) fn new (ocsp : & 'a [u8]) -> Self { CertificateStatus { ocsp_response : PayloadU24 :: from (Payload :: Borrowed (ocsp)) , } } pub (crate) fn into_inner (self) -> Vec < u8 > { self . ocsp_response . into_vec () } pub (crate) fn into_owned (self) -> CertificateStatus < 'static > { CertificateStatus { ocsp_response : self . ocsp_response . into_owned () , } } }
};
}
