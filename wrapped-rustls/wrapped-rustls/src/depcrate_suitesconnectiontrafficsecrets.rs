// Generated macro for ConnectionTrafficSecrets (enum)
macro_rules! Depcrate_suitesConnectionTrafficSecrets {
() => {
// Module: crate::suites
// Provides: {"ConnectionTrafficSecrets"}
// Dependencies: {}
# [doc = " Secrets used to encrypt/decrypt data in a TLS session."] # [doc = ""] # [doc = " These can be used to configure kTLS for a socket in one direction."] # [doc = " The only other piece of information needed is the sequence number,"] # [doc = " which is in [ExtractedSecrets]."] # [non_exhaustive] pub enum ConnectionTrafficSecrets { # [doc = " Secrets for the AES_128_GCM AEAD algorithm"] Aes128Gcm { # [doc = " AEAD Key"] key : AeadKey , # [doc = " Initialization vector"] iv : Iv , } , # [doc = " Secrets for the AES_256_GCM AEAD algorithm"] Aes256Gcm { # [doc = " AEAD Key"] key : AeadKey , # [doc = " Initialization vector"] iv : Iv , } , # [doc = " Secrets for the CHACHA20_POLY1305 AEAD algorithm"] Chacha20Poly1305 { # [doc = " AEAD Key"] key : AeadKey , # [doc = " Initialization vector"] iv : Iv , } , }
};
}
