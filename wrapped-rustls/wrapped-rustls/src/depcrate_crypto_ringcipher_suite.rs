// Generated macro for cipher_suite (module)
macro_rules! Depcrate_crypto_ringcipher_suite {
() => {
// Module: crate::crypto::ring
// Provides: {"cipher_suite"}
// Dependencies: {}
# [doc = " All defined cipher suites supported by *ring* appear in this module."] pub mod cipher_suite { pub use super :: tls12 :: { TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256 , TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384 , TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256 , TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256 , TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384 , TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256 , } ; pub use super :: tls13 :: { TLS13_AES_128_GCM_SHA256 , TLS13_AES_256_GCM_SHA384 , TLS13_CHACHA20_POLY1305_SHA256 , } ; }
};
}
