// Generated macro for CertificateStatus (struct)
macro_rules! Depcrate_msgs_handshakeCertificateStatus {
() => {
// Module: crate::msgs::handshake
// Provides: {"CertificateStatus"}
// Dependencies: {}
# [doc = " Only supports OCSP"] # [derive (Clone , Debug)] pub (crate) struct CertificateStatus < 'a > { # [doc = " `opaque OCSPResponse<1..2^24-1>;`"] pub (crate) ocsp_response : PayloadU24 < 'a , NonEmpty > , }
};
}
