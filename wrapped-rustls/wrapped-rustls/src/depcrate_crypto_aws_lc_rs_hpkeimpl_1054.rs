// Generated macro for impl_1054 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_hpkeimpl_1054 {
() => {
// Module: crate::crypto::aws_lc_rs::hpke
// Provides: {"impl_1054"}
// Dependencies: {}
impl < const KEY_SIZE : usize , const KDF_SIZE : usize > Hpke for HpkeAwsLcRs < KEY_SIZE , KDF_SIZE > { fn seal (& self , info : & [u8] , aad : & [u8] , plaintext : & [u8] , pub_key : & HpkePublicKey ,) -> Result < (EncapsulatedSecret , Vec < u8 >) , Error > { let (encap , mut sealer) = self . setup_sealer (info , pub_key) ? ; Ok ((encap , sealer . seal (aad , plaintext) ?)) } fn setup_sealer (& self , info : & [u8] , pub_key : & HpkePublicKey ,) -> Result < (EncapsulatedSecret , Box < dyn HpkeSealer + 'static >) , Error > { let (encap , sealer) = Sealer :: new (self , info , pub_key) ? ; Ok ((encap , Box :: new (sealer))) } fn open (& self , enc : & EncapsulatedSecret , info : & [u8] , aad : & [u8] , ciphertext : & [u8] , secret_key : & HpkePrivateKey ,) -> Result < Vec < u8 > , Error > { self . setup_opener (enc , info , secret_key) ? . open (aad , ciphertext) } fn setup_opener (& self , enc : & EncapsulatedSecret , info : & [u8] , secret_key : & HpkePrivateKey ,) -> Result < Box < dyn HpkeOpener + 'static > , Error > { Ok (Box :: new (Opener :: new (self , enc , info , secret_key) ?)) } fn fips (& self) -> bool { matches ! ((self . suite . kem , self . suite . sym . aead_id) , (HpkeKem :: DHKEM_P256_HKDF_SHA256 | HpkeKem :: DHKEM_P384_HKDF_SHA384 | HpkeKem :: DHKEM_P521_HKDF_SHA512 , HpkeAead :: AES_128_GCM | HpkeAead :: AES_256_GCM ,)) } fn generate_key_pair (& self) -> Result < (HpkePublicKey , HpkePrivateKey) , Error > { (self . dh_kem . key_generator) () } fn suite (& self) -> HpkeSuite { self . suite } }
};
}
