// Generated macro for generate_p_curve_key_pair (function)
macro_rules! Depcrate_crypto_aws_lc_rs_hpkegenerate_p_curve_key_pair {
() => {
// Module: crate::crypto::aws_lc_rs::hpke
// Provides: {"generate_p_curve_key_pair"}
// Dependencies: {}
# [doc = " Generate a NIST P-256, P-384 or P-512 key pair expressed as a raw big-endian fixed-length"] # [doc = " integer."] # [doc = ""] # [doc = " We must disambiguate the [`AsBigEndian`] trait in-use and this function uses"] # [doc = " [`AsBigEndian<EcPrivateKeyBin>`], which does not support [`agreement::X25519`]."] # [doc = " For generating X25519 keys see [`generate_x25519_key_pair`]."] fn generate_p_curve_key_pair (alg : & 'static agreement :: Algorithm ,) -> Result < (HpkePublicKey , HpkePrivateKey) , Error > { debug_assert_ne ! (alg , & agreement :: X25519) ; let (public_key , private_key) = generate_key_pair (alg) ? ; let raw_private_key : EcPrivateKeyBin < '_ > = private_key . as_be_bytes () . map_err (unspecified_err) ? ; Ok ((public_key , HpkePrivateKey :: from (raw_private_key . as_ref () . to_vec ()) ,)) }
};
}
