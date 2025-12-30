// Generated macro for impl_1074 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_hpkeimpl_1074 {
() => {
// Module: crate::crypto::aws_lc_rs::hpke
// Provides: {"impl_1074"}
// Dependencies: {}
impl < const KEY_SIZE : usize > KeySchedule < KEY_SIZE > { # [doc = " See [RFC 9180 §5.2 \"Encryption and Decryption\"][0]."] # [doc = ""] # [doc = " [0]: https://www.rfc-editor.org/rfc/rfc9180.html#section-5.2"] fn compute_nonce (& self) -> [u8 ; NONCE_LEN] { let mut nonce = [0 ; NONCE_LEN] ; let seq_bytes = self . seq_num . to_be_bytes () ; nonce [NONCE_LEN - seq_bytes . len () ..] . copy_from_slice (& seq_bytes) ; for (n , & b) in nonce . iter_mut () . zip (& self . base_nonce) { * n ^= b ; } nonce } # [doc = " See [RFC 9180 §5.2 \"Encryption and Decryption\"][0]."] # [doc = ""] # [doc = " [0]: https://www.rfc-editor.org/rfc/rfc9180.html#section-5.2"] fn increment_seq_num (& mut self) -> Result < () , aws_lc_rs :: error :: Unspecified > { let max_seq_num = (1u128 << (NONCE_LEN * 8)) - 1 ; if u128 :: from (self . seq_num) >= max_seq_num { return Err (aws_lc_rs :: error :: Unspecified) ; } self . seq_num += 1 ; Ok (()) } }
};
}
