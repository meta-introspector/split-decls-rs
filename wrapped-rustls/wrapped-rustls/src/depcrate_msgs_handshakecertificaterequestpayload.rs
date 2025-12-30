// Generated macro for CertificateRequestPayload (struct)
macro_rules! Depcrate_msgs_handshakeCertificateRequestPayload {
() => {
// Module: crate::msgs::handshake
// Provides: {"CertificateRequestPayload"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct CertificateRequestPayload { pub (crate) certtypes : Vec < ClientCertificateType > , pub (crate) sigschemes : Vec < SignatureScheme > , pub (crate) canames : Vec < DistinguishedName > , }
};
}
