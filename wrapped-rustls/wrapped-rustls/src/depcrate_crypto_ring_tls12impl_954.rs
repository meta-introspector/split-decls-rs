// Generated macro for impl_954 (impl)
macro_rules! Depcrate_crypto_ring_tls12impl_954 {
() => {
// Module: crate::crypto::ring::tls12
// Provides: {"impl_954"}
// Dependencies: {}
impl MessageEncrypter for ChaCha20Poly1305MessageEncrypter { fn encrypt (& mut self , msg : OutboundPlainMessage < '_ > , seq : u64 ,) -> Result < OutboundOpaqueMessage , Error > { let total_len = self . encrypted_payload_len (msg . payload . len ()) ; let mut payload = PrefixedPayload :: with_capacity (total_len) ; let nonce = aead :: Nonce :: assume_unique_for_key (Nonce :: new (& self . enc_offset , seq) . to_array () ?) ; let aad = aead :: Aad :: from (make_tls12_aad (seq , msg . typ , msg . version , msg . payload . len ())) ; payload . extend_from_chunks (& msg . payload) ; self . enc_key . seal_in_place_append_tag (nonce , aad , & mut payload) . map_err (| _ | Error :: EncryptError) ? ; Ok (OutboundOpaqueMessage { typ : msg . typ , version : msg . version , payload , }) } fn encrypted_payload_len (& self , payload_len : usize) -> usize { payload_len + self . enc_key . algorithm () . tag_len () } }
};
}
