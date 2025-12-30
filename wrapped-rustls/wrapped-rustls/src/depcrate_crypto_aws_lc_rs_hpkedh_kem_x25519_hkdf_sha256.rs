// Generated macro for DH_KEM_X25519_HKDF_SHA256 (static)
macro_rules! Depcrate_crypto_aws_lc_rs_hpkeDH_KEM_X25519_HKDF_SHA256 {
() => {
// Module: crate::crypto::aws_lc_rs::hpke
// Provides: {"DH_KEM_X25519_HKDF_SHA256"}
// Dependencies: {}
static DH_KEM_X25519_HKDF_SHA256 : & DhKem < SHA256_OUTPUT_LEN > = & DhKem { id : HpkeKem :: DHKEM_X25519_HKDF_SHA256 , agreement_algorithm : & agreement :: X25519 , key_generator : & generate_x25519_key_pair , hkdf : RING_HKDF_HMAC_SHA256 , } ;
};
}
