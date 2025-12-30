// Generated macro for sample_certificate_request_payload (function)
macro_rules! Depcrate_msgs_handshake_testsample_certificate_request_payload {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"sample_certificate_request_payload"}
// Dependencies: {}
fn sample_certificate_request_payload () -> CertificateRequestPayload { CertificateRequestPayload { certtypes : vec ! [ClientCertificateType :: RSASign] , sigschemes : vec ! [SignatureScheme :: ECDSA_NISTP256_SHA256] , canames : vec ! [DistinguishedName :: from (vec ! [1 , 2 , 3])] , } }
};
}
