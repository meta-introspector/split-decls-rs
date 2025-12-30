// Generated macro for DEFAULT_TLS12_PROVIDER (const)
macro_rules! Depcrate_crypto_aws_lc_rsDEFAULT_TLS12_PROVIDER {
() => {
// Module: crate::crypto::aws_lc_rs
// Provides: {"DEFAULT_TLS12_PROVIDER"}
// Dependencies: {}
# [doc = " The default `CryptoProvider` backed by aws-lc-rs that only supports TLS1.2."] # [doc = ""] # [doc = " Use of TLS1.3 is **strongly** recommended."] pub const DEFAULT_TLS12_PROVIDER : CryptoProvider = CryptoProvider { tls13_cipher_suites : Cow :: Borrowed (& []) , .. DEFAULT_PROVIDER } ;
};
}
