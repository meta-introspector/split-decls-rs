// Generated macro for generate_x25519_key_pair (function)
macro_rules! Depcrate_crypto_aws_lc_rs_hpkegenerate_x25519_key_pair {
() => {
// Module: crate::crypto::aws_lc_rs::hpke
// Provides: {"generate_x25519_key_pair"}
// Dependencies: {}
# [doc = " Generate a X25519 key pair expressed as a raw big-endian fixed-length"] # [doc = " integer."] # [doc = ""] # [doc = " We must disambiguate the [`AsBigEndian`] trait in-use and this function uses"] # [doc = " [`AsBigEndian<Curve25519SeedBin>`], which only supports [`agreement::X25519`]."] # [doc = " For generating P-256, P-384 and P-512 keys see [`generate_p_curve_key_pair`]."] fn generate_x25519_key_pair () -> Result < (HpkePublicKey , HpkePrivateKey) , Error > { let (public_key , private_key) = generate_key_pair (& agreement :: X25519) ? ; let raw_private_key : Curve25519SeedBin < '_ > = private_key . as_be_bytes () . map_err (unspecified_err) ? ; Ok ((public_key , HpkePrivateKey :: from (raw_private_key . as_ref () . to_vec ()) ,)) }
};
}
