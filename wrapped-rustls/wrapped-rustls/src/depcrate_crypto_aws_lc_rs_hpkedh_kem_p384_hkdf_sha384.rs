// Generated macro for DH_KEM_P384_HKDF_SHA384 (static)
macro_rules! Depcrate_crypto_aws_lc_rs_hpkeDH_KEM_P384_HKDF_SHA384 {
() => {
// Module: crate::crypto::aws_lc_rs::hpke
// Provides: {"DH_KEM_P384_HKDF_SHA384"}
// Dependencies: {}
static DH_KEM_P384_HKDF_SHA384 : & DhKem < SHA384_OUTPUT_LEN > = & DhKem { id : HpkeKem :: DHKEM_P384_HKDF_SHA384 , agreement_algorithm : & agreement :: ECDH_P384 , key_generator : & | | generate_p_curve_key_pair (& agreement :: ECDH_P384) , hkdf : RING_HKDF_HMAC_SHA384 , } ;
};
}
