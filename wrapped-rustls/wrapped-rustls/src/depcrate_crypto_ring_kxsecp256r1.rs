// Generated macro for SECP256R1 (static)
macro_rules! Depcrate_crypto_ring_kxSECP256R1 {
() => {
// Module: crate::crypto::ring::kx
// Provides: {"SECP256R1"}
// Dependencies: {}
# [doc = " Ephemeral ECDH on secp256r1 (aka NIST-P256)"] pub static SECP256R1 : & dyn SupportedKxGroup = & KxGroup { name : NamedGroup :: secp256r1 , agreement_algorithm : & agreement :: ECDH_P256 , fips_allowed : true , pub_key_validator : uncompressed_point , } ;
};
}
