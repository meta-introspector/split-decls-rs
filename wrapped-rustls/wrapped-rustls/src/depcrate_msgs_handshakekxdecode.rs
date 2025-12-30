// Generated macro for KxDecode (trait)
macro_rules! Depcrate_msgs_handshakeKxDecode {
() => {
// Module: crate::msgs::handshake
// Provides: {"KxDecode"}
// Dependencies: {}
pub (crate) trait KxDecode < 'a > : fmt :: Debug + Sized { # [doc = " Decode a key exchange message given the key_exchange `algo`"] fn decode (r : & mut Reader < 'a > , algo : KeyExchangeAlgorithm) -> Result < Self , InvalidMessage > ; }
};
}
