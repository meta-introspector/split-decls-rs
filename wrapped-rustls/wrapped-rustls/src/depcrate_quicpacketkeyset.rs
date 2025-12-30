// Generated macro for PacketKeySet (struct)
macro_rules! Depcrate_quicPacketKeySet {
() => {
// Module: crate::quic
// Provides: {"PacketKeySet"}
// Dependencies: {}
# [doc = " Packet protection keys for bidirectional 1-RTT communication"] # [expect (clippy :: exhaustive_structs)] pub struct PacketKeySet { # [doc = " Encrypts outgoing packets"] pub local : Box < dyn PacketKey > , # [doc = " Decrypts incoming packets"] pub remote : Box < dyn PacketKey > , }
};
}
