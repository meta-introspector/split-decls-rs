// Generated macro for X25519 (static)
macro_rules! Depcrate_crypto_aws_lc_rs_kxX25519 {
() => {
// Module: crate::crypto::aws_lc_rs::kx
// Provides: {"X25519"}
// Dependencies: {}
# [doc = " Ephemeral ECDH on curve25519 (see RFC7748)"] pub static X25519 : & dyn SupportedKxGroup = & KxGroup { name : NamedGroup :: X25519 , agreement_algorithm : & agreement :: X25519 , fips_allowed : false , pub_key_validator : | point : & [u8] | point . len () == 32 , } ;
};
}
