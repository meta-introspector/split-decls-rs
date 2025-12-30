// Generated macro for impl_1285 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_tls12impl_1285 {
() => {
// Module: crate::crypto::aws_lc_rs::tls12
// Provides: {"impl_1285"}
// Dependencies: {}
impl MessageDecrypter for ChaCha20Poly1305MessageDecrypter { fn decrypt < 'a > (& mut self , mut msg : InboundOpaqueMessage < 'a > , seq : u64 ,) -> Result < InboundPlainMessage < 'a > , Error > { let payload = & msg . payload ; if payload . len () < CHACHAPOLY1305_OVERHEAD { return Err (Error :: DecryptError) ; } let nonce = aead :: Nonce :: assume_unique_for_key (Nonce :: new (& self . dec_offset , seq) . to_array () ?) ; let aad = aead :: Aad :: from (make_tls12_aad (seq , msg . typ , msg . version , payload . len () - CHACHAPOLY1305_OVERHEAD ,)) ; let payload = & mut msg . payload ; let plain_len = self . dec_key . open_in_place (nonce , aad , payload) . map_err (| _ | Error :: DecryptError) ? . len () ; if plain_len > MAX_FRAGMENT_LEN { return Err (Error :: PeerSentOversizedRecord) ; } payload . truncate (plain_len) ; Ok (msg . into_plain_message ()) } }
};
}
