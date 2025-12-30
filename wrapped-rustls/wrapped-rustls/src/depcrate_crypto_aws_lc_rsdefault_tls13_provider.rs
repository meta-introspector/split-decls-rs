// Generated macro for DEFAULT_TLS13_PROVIDER (const)
macro_rules! Depcrate_crypto_aws_lc_rsDEFAULT_TLS13_PROVIDER {
() => {
// Module: crate::crypto::aws_lc_rs
// Provides: {"DEFAULT_TLS13_PROVIDER"}
// Dependencies: {}
# [doc = " The default `CryptoProvider` backed by aws-lc-rs that only supports TLS1.3."] pub const DEFAULT_TLS13_PROVIDER : CryptoProvider = CryptoProvider { tls12_cipher_suites : Cow :: Borrowed (& []) , .. DEFAULT_PROVIDER } ;
};
}
