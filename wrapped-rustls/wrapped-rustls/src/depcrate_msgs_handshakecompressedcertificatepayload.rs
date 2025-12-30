// Generated macro for CompressedCertificatePayload (struct)
macro_rules! Depcrate_msgs_handshakeCompressedCertificatePayload {
() => {
// Module: crate::msgs::handshake
// Provides: {"CompressedCertificatePayload"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct CompressedCertificatePayload < 'a > { pub (crate) alg : CertificateCompressionAlgorithm , pub (crate) uncompressed_len : u32 , # [doc = " `opaque compressed_certificate_message<1..2^24-1>;`"] pub (crate) compressed : PayloadU24 < 'a , NonEmpty > , }
};
}
