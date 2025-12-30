// Generated macro for impl_1062 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_hpkeimpl_1062 {
() => {
// Module: crate::crypto::aws_lc_rs::hpke
// Provides: {"impl_1062"}
// Dependencies: {}
impl < const KEY_SIZE : usize , const KDF_SIZE : usize > HpkeOpener for Opener < KEY_SIZE , KDF_SIZE > { fn open (& mut self , aad : & [u8] , ciphertext : & [u8]) -> Result < Vec < u8 > , Error > { let key = UnboundKey :: new (self . key_schedule . aead , & self . key_schedule . key . 0) . map_err (unspecified_err) ? ; let mut opening_key = OpeningKey :: new (key , & mut self . key_schedule) ; let mut in_out_buffer = Vec :: from (ciphertext) ; let plaintext = opening_key . open_in_place (Aad :: from (aad) , & mut in_out_buffer) . map_err (unspecified_err) ? ; Ok (plaintext . to_vec ()) } }
};
}
