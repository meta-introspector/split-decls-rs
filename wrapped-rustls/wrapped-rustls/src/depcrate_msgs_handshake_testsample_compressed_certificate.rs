// Generated macro for sample_compressed_certificate (function)
macro_rules! Depcrate_msgs_handshake_testsample_compressed_certificate {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"sample_compressed_certificate"}
// Dependencies: {}
fn sample_compressed_certificate () -> CompressedCertificatePayload < 'static > { CompressedCertificatePayload { alg : CertificateCompressionAlgorithm :: Brotli , uncompressed_len : 123 , compressed : PayloadU24 :: from (Payload :: new (vec ! [1 , 2 , 3])) , } }
};
}
