// Generated macro for PacketKey (struct)
macro_rules! Depcrate_crypto_ring_quicPacketKey {
() => {
// Module: crate::crypto::ring::quic
// Provides: {"PacketKey"}
// Dependencies: {}
pub (crate) struct PacketKey { # [doc = " Encrypts or decrypts a packet's payload"] key : aead :: LessSafeKey , # [doc = " Computes unique nonces for each packet"] iv : Iv , # [doc = " Confidentiality limit (see [`quic::PacketKey::confidentiality_limit`])"] confidentiality_limit : u64 , # [doc = " Integrity limit (see [`quic::PacketKey::integrity_limit`])"] integrity_limit : u64 , }
};
}
