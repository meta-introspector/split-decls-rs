// Generated macro for KeyBuilder (struct)
macro_rules! Depcrate_crypto_ring_quicKeyBuilder {
() => {
// Module: crate::crypto::ring::quic
// Provides: {"KeyBuilder"}
// Dependencies: {}
pub (crate) struct KeyBuilder { pub (crate) packet_alg : & 'static aead :: Algorithm , pub (crate) header_alg : & 'static aead :: quic :: Algorithm , pub (crate) confidentiality_limit : u64 , pub (crate) integrity_limit : u64 , }
};
}
