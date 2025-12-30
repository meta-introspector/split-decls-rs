// Generated macro for impl_891 (impl)
macro_rules! Depcrate_crypto_ring_quicimpl_891 {
() => {
// Module: crate::crypto::ring::quic
// Provides: {"impl_891"}
// Dependencies: {}
impl quic :: HeaderProtectionKey for HeaderProtectionKey { fn encrypt_in_place (& self , sample : & [u8] , first : & mut u8 , packet_number : & mut [u8] ,) -> Result < () , Error > { self . xor_in_place (sample , first , packet_number , false) } fn decrypt_in_place (& self , sample : & [u8] , first : & mut u8 , packet_number : & mut [u8] ,) -> Result < () , Error > { self . xor_in_place (sample , first , packet_number , true) } # [inline] fn sample_len (& self) -> usize { self . 0 . algorithm () . sample_len () } }
};
}
