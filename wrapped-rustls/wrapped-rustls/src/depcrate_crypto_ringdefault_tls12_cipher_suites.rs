// Generated macro for DEFAULT_TLS12_CIPHER_SUITES (static)
macro_rules! Depcrate_crypto_ringDEFAULT_TLS12_CIPHER_SUITES {
() => {
// Module: crate::crypto::ring
// Provides: {"DEFAULT_TLS12_CIPHER_SUITES"}
// Dependencies: {}
# [doc = " The TLS1.2 cipher suite configuration that an application should use by default."] # [doc = ""] # [doc = " This will be [`ALL_TLS12_CIPHER_SUITES`] sans any supported cipher suites that"] # [doc = " shouldn't be enabled by most applications."] pub static DEFAULT_TLS12_CIPHER_SUITES : & [& Tls12CipherSuite] = ALL_TLS12_CIPHER_SUITES ;
};
}
