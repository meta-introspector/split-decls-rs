// Generated macro for sample_certificate_payload_tls13 (function)
macro_rules! Depcrate_msgs_handshake_testsample_certificate_payload_tls13 {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"sample_certificate_payload_tls13"}
// Dependencies: {}
fn sample_certificate_payload_tls13 () -> CertificatePayloadTls13 < 'static > { CertificatePayloadTls13 { context : PayloadU8 :: new (vec ! [1 , 2 , 3]) , entries : vec ! [CertificateEntry { cert : CertificateDer :: from (vec ! [3 , 4 , 5]) , extensions : CertificateExtensions { status : Some (CertificateStatus { ocsp_response : PayloadU24 :: from (Payload :: new (vec ! [1 , 2 , 3])) , }) , } , }] , } }
};
}
