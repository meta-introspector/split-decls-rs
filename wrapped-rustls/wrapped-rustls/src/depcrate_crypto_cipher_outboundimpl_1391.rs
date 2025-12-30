// Generated macro for impl_1391 (impl)
macro_rules! Depcrate_crypto_cipher_outboundimpl_1391 {
() => {
// Module: crate::crypto::cipher::outbound
// Provides: {"impl_1391"}
// Dependencies: {}
impl OutboundOpaqueMessage { # [doc = " Encode this message to a vector of bytes."] pub fn encode (self) -> Vec < u8 > { let length = self . payload . len () as u16 ; let mut encoded_payload = self . payload . 0 ; encoded_payload [0] = self . typ . into () ; encoded_payload [1 .. 3] . copy_from_slice (& self . version . to_array ()) ; encoded_payload [3 .. 5] . copy_from_slice (& (length) . to_be_bytes ()) ; encoded_payload } }
};
}
