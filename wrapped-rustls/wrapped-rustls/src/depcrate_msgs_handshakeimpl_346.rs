// Generated macro for impl_346 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_346 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_346"}
// Dependencies: {}
impl KxDecode < '_ > for ServerKeyExchangeParams { fn decode (r : & mut Reader < '_ > , algo : KeyExchangeAlgorithm) -> Result < Self , InvalidMessage > { use KeyExchangeAlgorithm :: * ; Ok (match algo { ECDHE => Self :: Ecdh (ServerEcdhParams :: read (r) ?) , DHE => Self :: Dh (ServerDhParams :: read (r) ?) , }) } }
};
}
