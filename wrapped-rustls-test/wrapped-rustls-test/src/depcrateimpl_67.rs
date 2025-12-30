// Generated macro for impl_67 (impl)
macro_rules! Depcrateimpl_67 {
() => {
// Module: crate
// Provides: {"impl_67"}
// Dependencies: {}
impl Default for MockServerVerifier { fn default () -> Self { Self { cert_rejection_error : None , tls12_signature_error : None , tls13_signature_error : None , signature_schemes : vec ! [SignatureScheme :: RSA_PSS_SHA256 , SignatureScheme :: RSA_PKCS1_SHA256 , SignatureScheme :: ED25519 , SignatureScheme :: ECDSA_NISTP256_SHA256 , SignatureScheme :: ECDSA_NISTP384_SHA384 , SignatureScheme :: ECDSA_NISTP521_SHA512 ,] , expected_ocsp_response : None , requires_raw_public_keys : false , raw_public_key_algorithms : None , } } }
};
}
