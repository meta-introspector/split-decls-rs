// Generated macro for DH_KEM_P521_HKDF_SHA512 (static)
macro_rules! Depcrate_crypto_aws_lc_rs_hpkeDH_KEM_P521_HKDF_SHA512 {
() => {
// Module: crate::crypto::aws_lc_rs::hpke
// Provides: {"DH_KEM_P521_HKDF_SHA512"}
// Dependencies: {}
static DH_KEM_P521_HKDF_SHA512 : & DhKem < SHA512_OUTPUT_LEN > = & DhKem { id : HpkeKem :: DHKEM_P521_HKDF_SHA512 , agreement_algorithm : & agreement :: ECDH_P521 , key_generator : & | | generate_p_curve_key_pair (& agreement :: ECDH_P521) , hkdf : RING_HKDF_HMAC_SHA512 , } ;
};
}
