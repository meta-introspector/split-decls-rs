// Generated macro for impl_982 (impl)
macro_rules! Depcrate_crypto_ring_tls13impl_982 {
() => {
// Module: crate::crypto::ring::tls13
// Provides: {"impl_982"}
// Dependencies: {}
impl MessageDecrypter for Tls13MessageDecrypter { fn decrypt < 'a > (& mut self , mut msg : InboundOpaqueMessage < 'a > , seq : u64 ,) -> Result < InboundPlainMessage < 'a > , Error > { let payload = & mut msg . payload ; if payload . len () < self . dec_key . algorithm () . tag_len () { return Err (Error :: DecryptError) ; } let nonce = aead :: Nonce :: assume_unique_for_key (Nonce :: new (& self . iv , seq) . to_array () ?) ; let aad = aead :: Aad :: from (make_tls13_aad (payload . len ())) ; let plain_len = self . dec_key . open_in_place (nonce , aad , payload) . map_err (| _ | Error :: DecryptError) ? . len () ; payload . truncate (plain_len) ; msg . into_tls13_unpadded_message () } }
};
}
