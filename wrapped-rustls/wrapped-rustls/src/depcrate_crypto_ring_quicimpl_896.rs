// Generated macro for impl_896 (impl)
macro_rules! Depcrate_crypto_ring_quicimpl_896 {
() => {
// Module: crate::crypto::ring::quic
// Provides: {"impl_896"}
// Dependencies: {}
impl quic :: Algorithm for KeyBuilder { fn packet_key (& self , key : AeadKey , iv : Iv) -> Box < dyn quic :: PacketKey > { Box :: new (PacketKey :: new (key , iv , self . confidentiality_limit , self . integrity_limit , self . packet_alg ,)) } fn header_protection_key (& self , key : AeadKey) -> Box < dyn quic :: HeaderProtectionKey > { Box :: new (HeaderProtectionKey :: new (key , self . header_alg)) } fn aead_key_len (& self) -> usize { self . packet_alg . key_len () } fn fips (& self) -> bool { super :: fips () } }
};
}
