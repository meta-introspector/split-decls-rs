// Generated macro for DirectionalKeys (struct)
macro_rules! Depcrate_quicDirectionalKeys {
() => {
// Module: crate::quic
// Provides: {"DirectionalKeys"}
// Dependencies: {}
# [doc = " Keys used to communicate in a single direction"] # [expect (clippy :: exhaustive_structs)] pub struct DirectionalKeys { # [doc = " Encrypts or decrypts a packet's headers"] pub header : Box < dyn HeaderProtectionKey > , # [doc = " Encrypts or decrypts the payload of a packet"] pub packet : Box < dyn PacketKey > , }
};
}
