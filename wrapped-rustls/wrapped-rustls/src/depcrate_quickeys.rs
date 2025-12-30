// Generated macro for Keys (struct)
macro_rules! Depcrate_quicKeys {
() => {
// Module: crate::quic
// Provides: {"Keys"}
// Dependencies: {}
# [doc = " Complete set of keys used to communicate with the peer"] # [expect (clippy :: exhaustive_structs)] pub struct Keys { # [doc = " Encrypts outgoing packets"] pub local : DirectionalKeys , # [doc = " Decrypts incoming packets"] pub remote : DirectionalKeys , }
};
}
