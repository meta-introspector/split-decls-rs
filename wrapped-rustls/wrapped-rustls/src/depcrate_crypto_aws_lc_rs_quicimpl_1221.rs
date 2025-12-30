// Generated macro for impl_1221 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_quicimpl_1221 {
() => {
// Module: crate::crypto::aws_lc_rs::quic
// Provides: {"impl_1221"}
// Dependencies: {}
impl quic :: HeaderProtectionKey for HeaderProtectionKey { fn encrypt_in_place (& self , sample : & [u8] , first : & mut u8 , packet_number : & mut [u8] ,) -> Result < () , Error > { self . xor_in_place (sample , first , packet_number , false) } fn decrypt_in_place (& self , sample : & [u8] , first : & mut u8 , packet_number : & mut [u8] ,) -> Result < () , Error > { self . xor_in_place (sample , first , packet_number , true) } # [inline] fn sample_len (& self) -> usize { self . 0 . algorithm () . sample_len () } }
};
}
