// Generated macro for ExtractedSecrets (struct)
macro_rules! Depcrate_suitesExtractedSecrets {
() => {
// Module: crate::suites
// Provides: {"ExtractedSecrets"}
// Dependencies: {}
# [doc = " Secrets for transmitting/receiving data over a TLS session."] # [doc = ""] # [doc = " After performing a handshake with rustls, these secrets can be extracted"] # [doc = " to configure kTLS for a socket, and have the kernel take over encryption"] # [doc = " and/or decryption."] # [expect (clippy :: exhaustive_structs)] pub struct ExtractedSecrets { # [doc = " sequence number and secrets for the \"tx\" (transmit) direction"] pub tx : (u64 , ConnectionTrafficSecrets) , # [doc = " sequence number and secrets for the \"rx\" (receive) direction"] pub rx : (u64 , ConnectionTrafficSecrets) , }
};
}
