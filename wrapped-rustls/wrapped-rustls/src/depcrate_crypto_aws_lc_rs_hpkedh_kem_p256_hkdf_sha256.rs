// Generated macro for DH_KEM_P256_HKDF_SHA256 (static)
macro_rules! Depcrate_crypto_aws_lc_rs_hpkeDH_KEM_P256_HKDF_SHA256 {
() => {
// Module: crate::crypto::aws_lc_rs::hpke
// Provides: {"DH_KEM_P256_HKDF_SHA256"}
// Dependencies: {}
static DH_KEM_P256_HKDF_SHA256 : & DhKem < SHA256_OUTPUT_LEN > = & DhKem { id : HpkeKem :: DHKEM_P256_HKDF_SHA256 , agreement_algorithm : & agreement :: ECDH_P256 , key_generator : & | | generate_p_curve_key_pair (& agreement :: ECDH_P256) , hkdf : RING_HKDF_HMAC_SHA256 , } ;
};
}
