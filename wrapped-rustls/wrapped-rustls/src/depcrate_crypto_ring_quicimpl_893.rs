// Generated macro for impl_893 (impl)
macro_rules! Depcrate_crypto_ring_quicimpl_893 {
() => {
// Module: crate::crypto::ring::quic
// Provides: {"impl_893"}
// Dependencies: {}
impl PacketKey { pub (crate) fn new (key : AeadKey , iv : Iv , confidentiality_limit : u64 , integrity_limit : u64 , aead_algorithm : & 'static aead :: Algorithm ,) -> Self { Self { key : aead :: LessSafeKey :: new (aead :: UnboundKey :: new (aead_algorithm , key . as_ref ()) . unwrap () ,) , iv , confidentiality_limit , integrity_limit , } } }
};
}
