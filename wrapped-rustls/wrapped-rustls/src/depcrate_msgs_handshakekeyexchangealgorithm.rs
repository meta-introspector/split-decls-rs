// Generated macro for KeyExchangeAlgorithm (enum)
macro_rules! Depcrate_msgs_handshakeKeyExchangeAlgorithm {
() => {
// Module: crate::msgs::handshake
// Provides: {"KeyExchangeAlgorithm"}
// Dependencies: {}
# [doc = " Describes supported key exchange mechanisms."] # [derive (Clone , Copy , Debug , PartialEq)] # [non_exhaustive] pub enum KeyExchangeAlgorithm { # [doc = " Diffie-Hellman Key exchange (with only known parameters as defined in [RFC 7919])."] # [doc = ""] # [doc = " [RFC 7919]: https://datatracker.ietf.org/doc/html/rfc7919"] DHE , # [doc = " Key exchange performed via elliptic curve Diffie-Hellman."] ECDHE , }
};
}
