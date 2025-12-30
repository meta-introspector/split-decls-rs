// Generated macro for DEFAULT_TLS13_CIPHER_SUITES (static)
macro_rules! Depcrate_crypto_aws_lc_rsDEFAULT_TLS13_CIPHER_SUITES {
() => {
// Module: crate::crypto::aws_lc_rs
// Provides: {"DEFAULT_TLS13_CIPHER_SUITES"}
// Dependencies: {}
# [doc = " The TLS1.3 cipher suite configuration that an application should use by default."] # [doc = ""] # [doc = " This will be [`ALL_TLS13_CIPHER_SUITES`] sans any supported cipher suites that"] # [doc = " shouldn't be enabled by most applications."] pub static DEFAULT_TLS13_CIPHER_SUITES : & [& Tls13CipherSuite] = & [tls13 :: TLS13_AES_256_GCM_SHA384 , tls13 :: TLS13_AES_128_GCM_SHA256 , # [cfg (not (feature = "fips"))] tls13 :: TLS13_CHACHA20_POLY1305_SHA256 ,] ;
};
}
