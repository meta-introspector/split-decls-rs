// Generated macro for MockServerVerifier (struct)
macro_rules! DepcrateMockServerVerifier {
() => {
// Module: crate
// Provides: {"MockServerVerifier"}
// Dependencies: {}
# [derive (Debug)] pub struct MockServerVerifier { cert_rejection_error : Option < Error > , tls12_signature_error : Option < Error > , tls13_signature_error : Option < Error > , signature_schemes : Vec < SignatureScheme > , expected_ocsp_response : Option < Vec < u8 > > , requires_raw_public_keys : bool , raw_public_key_algorithms : Option < WebPkiSupportedAlgorithms > , }
};
}
