// Generated macro for DEFAULT_TLS12_CIPHER_SUITES (static)
macro_rules! Depcrate_crypto_aws_lc_rsDEFAULT_TLS12_CIPHER_SUITES {
() => {
// Module: crate::crypto::aws_lc_rs
// Provides: {"DEFAULT_TLS12_CIPHER_SUITES"}
// Dependencies: {}
# [doc = " The TLS1.2 cipher suite configuration that an application should use by default."] # [doc = ""] # [doc = " This will be [`ALL_TLS12_CIPHER_SUITES`] sans any supported cipher suites that"] # [doc = " shouldn't be enabled by most applications."] pub static DEFAULT_TLS12_CIPHER_SUITES : & [& Tls12CipherSuite] = & [tls12 :: TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384 , tls12 :: TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256 , # [cfg (not (feature = "fips"))] tls12 :: TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256 , tls12 :: TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384 , tls12 :: TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256 , # [cfg (not (feature = "fips"))] tls12 :: TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256 ,] ;
};
}
