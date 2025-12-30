// Generated macro for impl_161 (impl)
macro_rules! Depcrate_msgs_enumsimpl_161 {
() => {
// Module: crate::msgs::enums
// Provides: {"impl_161"}
// Dependencies: {}
impl NamedGroup { # [doc = " Return the key exchange algorithm associated with this `NamedGroup`"] pub fn key_exchange_algorithm (self) -> KeyExchangeAlgorithm { match u16 :: from (self) { x if (0x100 .. 0x200) . contains (& x) => KeyExchangeAlgorithm :: DHE , _ => KeyExchangeAlgorithm :: ECDHE , } } pub fn usable_for_version (& self , version : ProtocolVersion) -> bool { match version { ProtocolVersion :: TLSv1_3 => true , _ => ! matches ! (self , Self :: MLKEM512 | Self :: MLKEM768 | Self :: MLKEM1024 | Self :: X25519MLKEM768 | Self :: secp256r1MLKEM768 | Self :: secp384r1MLKEM1024 | Self :: brainpoolP256r1tls13 | Self :: brainpoolP384r1tls13 | Self :: brainpoolP512r1tls13 | Self :: curveSM2) , } } }
};
}
