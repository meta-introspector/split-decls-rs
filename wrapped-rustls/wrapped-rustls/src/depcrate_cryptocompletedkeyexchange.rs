// Generated macro for CompletedKeyExchange (struct)
macro_rules! Depcrate_cryptoCompletedKeyExchange {
() => {
// Module: crate::crypto
// Provides: {"CompletedKeyExchange"}
// Dependencies: {}
# [doc = " The result from [`SupportedKxGroup::start_and_complete()`]."] # [expect (clippy :: exhaustive_structs)] pub struct CompletedKeyExchange { # [doc = " Which group was used."] pub group : NamedGroup , # [doc = " Our key share (sometimes a public key)."] pub pub_key : Vec < u8 > , # [doc = " The computed shared secret."] pub secret : SharedSecret , }
};
}
