// Generated macro for impl_1280 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_tls12impl_1280 {
() => {
// Module: crate::crypto::aws_lc_rs::tls12
// Provides: {"impl_1280"}
// Dependencies: {}
impl MessageDecrypter for GcmMessageDecrypter { fn decrypt < 'a > (& mut self , mut msg : InboundOpaqueMessage < 'a > , seq : u64 ,) -> Result < InboundPlainMessage < 'a > , Error > { let payload = & msg . payload ; if payload . len () < GCM_OVERHEAD { return Err (Error :: DecryptError) ; } let nonce = { let mut nonce = [0u8 ; 12] ; nonce [.. 4] . copy_from_slice (& self . dec_salt) ; nonce [4 ..] . copy_from_slice (& payload [.. 8]) ; aead :: Nonce :: assume_unique_for_key (nonce) } ; let aad = aead :: Aad :: from (make_tls12_aad (seq , msg . typ , msg . version , payload . len () - GCM_OVERHEAD ,)) ; let payload = & mut msg . payload ; let plain_len = self . dec_key . open_in_place (nonce , aad , & mut payload [GCM_EXPLICIT_NONCE_LEN ..]) . map_err (| _ | Error :: DecryptError) ? . len () ; if plain_len > MAX_FRAGMENT_LEN { return Err (Error :: PeerSentOversizedRecord) ; } Ok (msg . into_plain_message_range (GCM_EXPLICIT_NONCE_LEN .. GCM_EXPLICIT_NONCE_LEN + plain_len ,) ,) } }
};
}
