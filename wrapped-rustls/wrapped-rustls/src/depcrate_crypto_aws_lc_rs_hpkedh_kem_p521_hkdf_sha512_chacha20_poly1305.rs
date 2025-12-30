// Generated macro for DH_KEM_P521_HKDF_SHA512_CHACHA20_POLY1305 (static)
macro_rules! Depcrate_crypto_aws_lc_rs_hpkeDH_KEM_P521_HKDF_SHA512_CHACHA20_POLY1305 {
() => {
// Module: crate::crypto::aws_lc_rs::hpke
// Provides: {"DH_KEM_P521_HKDF_SHA512_CHACHA20_POLY1305"}
// Dependencies: {}
# [doc = " HPKE suite using ECDH P-521 for agreement, HKDF SHA-512 for key derivation, and AEAD"] # [doc = " CHACHA20-POLY-1305 for symmetric encryption."] pub static DH_KEM_P521_HKDF_SHA512_CHACHA20_POLY1305 : & HpkeAwsLcRs < CHACHA_KEY_LEN , SHA512_OUTPUT_LEN , > = & HpkeAwsLcRs { suite : HpkeSuite { kem : HpkeKem :: DHKEM_P521_HKDF_SHA512 , sym : HpkeSymmetricCipherSuite { kdf_id : HpkeKdf :: HKDF_SHA512 , aead_id : HpkeAead :: CHACHA20_POLY_1305 , } , } , dh_kem : DH_KEM_P521_HKDF_SHA512 , hkdf : RING_HKDF_HMAC_SHA512 , aead : & aead :: CHACHA20_POLY1305 , } ;
};
}
