// Generated macro for impl_1319 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_tls13impl_1319 {
() => {
// Module: crate::crypto::aws_lc_rs::tls13
// Provides: {"impl_1319"}
// Dependencies: {}
impl MessageEncrypter for AeadMessageEncrypter { fn encrypt (& mut self , msg : OutboundPlainMessage < '_ > , seq : u64 ,) -> Result < OutboundOpaqueMessage , Error > { let total_len = self . encrypted_payload_len (msg . payload . len ()) ; let mut payload = PrefixedPayload :: with_capacity (total_len) ; let nonce = aead :: Nonce :: assume_unique_for_key (Nonce :: new (& self . iv , seq) . to_array () ?) ; let aad = aead :: Aad :: from (make_tls13_aad (total_len)) ; payload . extend_from_chunks (& msg . payload) ; payload . extend_from_slice (& msg . typ . to_array ()) ; self . enc_key . seal_in_place_append_tag (nonce , aad , & mut payload) . map_err (| _ | Error :: EncryptError) ? ; Ok (OutboundOpaqueMessage { typ : ContentType :: ApplicationData , version : ProtocolVersion :: TLSv1_2 , payload , }) } fn encrypted_payload_len (& self , payload_len : usize) -> usize { payload_len + 1 + self . enc_key . algorithm () . tag_len () } }
};
}
