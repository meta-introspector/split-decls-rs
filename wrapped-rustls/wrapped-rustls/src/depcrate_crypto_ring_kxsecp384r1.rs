// Generated macro for SECP384R1 (static)
macro_rules! Depcrate_crypto_ring_kxSECP384R1 {
() => {
// Module: crate::crypto::ring::kx
// Provides: {"SECP384R1"}
// Dependencies: {}
# [doc = " Ephemeral ECDH on secp384r1 (aka NIST-P384)"] pub static SECP384R1 : & dyn SupportedKxGroup = & KxGroup { name : NamedGroup :: secp384r1 , agreement_algorithm : & agreement :: ECDH_P384 , fips_allowed : true , pub_key_validator : uncompressed_point , } ;
};
}
