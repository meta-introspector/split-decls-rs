// Generated macro for CertificateStatusRequest (enum)
macro_rules! Depcrate_msgs_handshakeCertificateStatusRequest {
() => {
// Module: crate::msgs::handshake
// Provides: {"CertificateStatusRequest"}
// Dependencies: {}
# [derive (Clone , Debug)] pub (crate) enum CertificateStatusRequest { Ocsp (OcspCertificateStatusRequest) , Unknown ((CertificateStatusType , Payload < 'static >)) , }
};
}
