// Generated macro for impl_333 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_333 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_333"}
// Dependencies: {}
impl KxDecode < '_ > for ClientKeyExchangeParams { fn decode (r : & mut Reader < '_ > , algo : KeyExchangeAlgorithm) -> Result < Self , InvalidMessage > { use KeyExchangeAlgorithm :: * ; Ok (match algo { ECDHE => Self :: Ecdh (ClientEcdhParams :: read (r) ?) , DHE => Self :: Dh (ClientDhParams :: read (r) ?) , }) } }
};
}
