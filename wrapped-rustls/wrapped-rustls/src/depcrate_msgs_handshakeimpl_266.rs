// Generated macro for impl_266 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_266 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_266"}
// Dependencies: {}
impl CertificateStatusRequest { pub (crate) fn build_ocsp () -> Self { let ocsp = OcspCertificateStatusRequest { responder_ids : Vec :: new () , extensions : PayloadU16 :: empty () , } ; Self :: Ocsp (ocsp) } }
};
}
