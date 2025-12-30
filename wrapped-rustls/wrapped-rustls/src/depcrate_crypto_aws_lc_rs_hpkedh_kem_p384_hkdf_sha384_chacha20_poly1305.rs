// Generated macro for DH_KEM_P384_HKDF_SHA384_CHACHA20_POLY1305 (static)
macro_rules! Depcrate_crypto_aws_lc_rs_hpkeDH_KEM_P384_HKDF_SHA384_CHACHA20_POLY1305 {
() => {
// Module: crate::crypto::aws_lc_rs::hpke
// Provides: {"DH_KEM_P384_HKDF_SHA384_CHACHA20_POLY1305"}
// Dependencies: {}
# [doc = " HPKE suite using ECDH P-384 for agreement, HKDF SHA-384 for key derivation, and AEAD"] # [doc = " CHACHA20-POLY-1305 for symmetric encryption."] pub static DH_KEM_P384_HKDF_SHA384_CHACHA20_POLY1305 : & HpkeAwsLcRs < CHACHA_KEY_LEN , SHA384_OUTPUT_LEN , > = & HpkeAwsLcRs { suite : HpkeSuite { kem : HpkeKem :: DHKEM_P384_HKDF_SHA384 , sym : HpkeSymmetricCipherSuite { kdf_id : HpkeKdf :: HKDF_SHA384 , aead_id : HpkeAead :: CHACHA20_POLY_1305 , } , } , dh_kem : DH_KEM_P384_HKDF_SHA384 , hkdf : RING_HKDF_HMAC_SHA384 , aead : & aead :: CHACHA20_POLY1305 , } ;
};
}
