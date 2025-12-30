// Generated macro for DH_KEM_P521_HKDF_SHA512_AES_256 (static)
macro_rules! Depcrate_crypto_aws_lc_rs_hpkeDH_KEM_P521_HKDF_SHA512_AES_256 {
() => {
// Module: crate::crypto::aws_lc_rs::hpke
// Provides: {"DH_KEM_P521_HKDF_SHA512_AES_256"}
// Dependencies: {}
# [doc = " HPKE suite using ECDH P-521 for agreement, HKDF SHA-512 for key derivation, and AEAD AES-256-GCM"] # [doc = " for symmetric encryption."] pub static DH_KEM_P521_HKDF_SHA512_AES_256 : & HpkeAwsLcRs < AES_256_KEY_LEN , SHA512_OUTPUT_LEN > = & HpkeAwsLcRs { suite : HpkeSuite { kem : HpkeKem :: DHKEM_P521_HKDF_SHA512 , sym : HpkeSymmetricCipherSuite { kdf_id : HpkeKdf :: HKDF_SHA512 , aead_id : HpkeAead :: AES_256_GCM , } , } , dh_kem : DH_KEM_P521_HKDF_SHA512 , hkdf : RING_HKDF_HMAC_SHA512 , aead : & aead :: AES_256_GCM , } ;
};
}
