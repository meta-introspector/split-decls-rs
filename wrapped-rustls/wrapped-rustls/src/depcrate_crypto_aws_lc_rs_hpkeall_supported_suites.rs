// Generated macro for ALL_SUPPORTED_SUITES (static)
macro_rules! Depcrate_crypto_aws_lc_rs_hpkeALL_SUPPORTED_SUITES {
() => {
// Module: crate::crypto::aws_lc_rs::hpke
// Provides: {"ALL_SUPPORTED_SUITES"}
// Dependencies: {}
# [doc = " Default [RFC 9180] Hybrid Public Key Encryption (HPKE) suites supported by aws-lc-rs cryptography."] pub static ALL_SUPPORTED_SUITES : & [& dyn Hpke] = & [DH_KEM_P256_HKDF_SHA256_AES_128 , DH_KEM_P256_HKDF_SHA256_AES_256 , # [cfg (not (feature = "fips"))] DH_KEM_P256_HKDF_SHA256_CHACHA20_POLY1305 , DH_KEM_P384_HKDF_SHA384_AES_128 , DH_KEM_P384_HKDF_SHA384_AES_256 , # [cfg (not (feature = "fips"))] DH_KEM_P384_HKDF_SHA384_CHACHA20_POLY1305 , DH_KEM_P521_HKDF_SHA512_AES_128 , DH_KEM_P521_HKDF_SHA512_AES_256 , # [cfg (not (feature = "fips"))] DH_KEM_P521_HKDF_SHA512_CHACHA20_POLY1305 , # [cfg (not (feature = "fips"))] DH_KEM_X25519_HKDF_SHA256_AES_128 , # [cfg (not (feature = "fips"))] DH_KEM_X25519_HKDF_SHA256_AES_256 , # [cfg (not (feature = "fips"))] DH_KEM_X25519_HKDF_SHA256_CHACHA20_POLY1305 ,] ;
};
}
