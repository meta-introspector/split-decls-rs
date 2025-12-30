// Generated macro for sample_certificate_request_payload_tls13 (function)
macro_rules! Depcrate_msgs_handshake_testsample_certificate_request_payload_tls13 {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"sample_certificate_request_payload_tls13"}
// Dependencies: {}
fn sample_certificate_request_payload_tls13 () -> CertificateRequestPayloadTls13 { CertificateRequestPayloadTls13 { context : PayloadU8 :: new (vec ! [1 , 2 , 3]) , extensions : CertificateRequestExtensions { signature_algorithms : Some (vec ! [SignatureScheme :: ECDSA_NISTP256_SHA256]) , authority_names : Some (vec ! [DistinguishedName :: from (vec ! [1 , 2 , 3])]) , certificate_compression_algorithms : Some (vec ! [CertificateCompressionAlgorithm :: Zlib]) , } , } }
};
}
