// Generated macro for impl_1226 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_quicimpl_1226 {
() => {
// Module: crate::crypto::aws_lc_rs::quic
// Provides: {"impl_1226"}
// Dependencies: {}
impl quic :: Algorithm for KeyBuilder { fn packet_key (& self , key : AeadKey , iv : Iv) -> Box < dyn quic :: PacketKey > { Box :: new (PacketKey :: new (key , iv , self . confidentiality_limit , self . integrity_limit , self . packet_alg ,)) } fn header_protection_key (& self , key : AeadKey) -> Box < dyn quic :: HeaderProtectionKey > { Box :: new (HeaderProtectionKey :: new (key , self . header_alg)) } fn aead_key_len (& self) -> usize { self . packet_alg . key_len () } fn fips (& self) -> bool { super :: fips () } }
};
}
