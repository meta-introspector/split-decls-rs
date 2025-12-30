// Generated macro for impl_949 (impl)
macro_rules! Depcrate_crypto_ring_tls12impl_949 {
() => {
// Module: crate::crypto::ring::tls12
// Provides: {"impl_949"}
// Dependencies: {}
impl MessageEncrypter for GcmMessageEncrypter { fn encrypt (& mut self , msg : OutboundPlainMessage < '_ > , seq : u64 ,) -> Result < OutboundOpaqueMessage , Error > { let total_len = self . encrypted_payload_len (msg . payload . len ()) ; let mut payload = PrefixedPayload :: with_capacity (total_len) ; let nonce = aead :: Nonce :: assume_unique_for_key (Nonce :: new (& self . iv , seq) . to_array () ?) ; let aad = aead :: Aad :: from (make_tls12_aad (seq , msg . typ , msg . version , msg . payload . len ())) ; payload . extend_from_slice (& nonce . as_ref () [4 ..]) ; payload . extend_from_chunks (& msg . payload) ; self . enc_key . seal_in_place_separate_tag (nonce , aad , & mut payload . as_mut () [GCM_EXPLICIT_NONCE_LEN ..]) . map (| tag | payload . extend_from_slice (tag . as_ref ())) . map_err (| _ | Error :: EncryptError) ? ; Ok (OutboundOpaqueMessage { typ : msg . typ , version : msg . version , payload , }) } fn encrypted_payload_len (& self , payload_len : usize) -> usize { payload_len + GCM_EXPLICIT_NONCE_LEN + self . enc_key . algorithm () . tag_len () } }
};
}
