// Generated macro for impl_1058 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_hpkeimpl_1058 {
() => {
// Module: crate::crypto::aws_lc_rs::hpke
// Provides: {"impl_1058"}
// Dependencies: {}
impl < const KEY_SIZE : usize , const KDF_SIZE : usize > HpkeSealer for Sealer < KEY_SIZE , KDF_SIZE > { fn seal (& mut self , aad : & [u8] , plaintext : & [u8]) -> Result < Vec < u8 > , Error > { let key = UnboundKey :: new (self . key_schedule . aead , & self . key_schedule . key . 0) . map_err (unspecified_err) ? ; let mut sealing_key = SealingKey :: new (key , & mut self . key_schedule) ; let mut in_out_buffer = Vec :: from (plaintext) ; sealing_key . seal_in_place_append_tag (Aad :: from (aad) , & mut in_out_buffer) . map_err (unspecified_err) ? ; Ok (in_out_buffer) } }
};
}
