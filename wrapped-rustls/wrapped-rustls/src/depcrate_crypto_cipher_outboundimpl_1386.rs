// Generated macro for impl_1386 (impl)
macro_rules! Depcrate_crypto_cipher_outboundimpl_1386 {
() => {
// Module: crate::crypto::cipher::outbound
// Provides: {"impl_1386"}
// Dependencies: {}
impl OutboundPlainMessage < '_ > { pub (crate) fn encoded_len (& self , record_layer : & RecordLayer) -> usize { HEADER_SIZE + record_layer . encrypted_len (self . payload . len ()) } pub (crate) fn to_unencrypted_opaque (& self) -> OutboundOpaqueMessage { let mut payload = PrefixedPayload :: with_capacity (self . payload . len ()) ; payload . extend_from_chunks (& self . payload) ; OutboundOpaqueMessage { version : self . version , typ : self . typ , payload , } } }
};
}
