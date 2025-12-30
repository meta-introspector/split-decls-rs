// Generated macro for DEFAULT_TLS13_PROVIDER (const)
macro_rules! Depcrate_crypto_ringDEFAULT_TLS13_PROVIDER {
() => {
// Module: crate::crypto::ring
// Provides: {"DEFAULT_TLS13_PROVIDER"}
// Dependencies: {}
# [doc = " The default `CryptoProvider` backed by *ring* that only supports TLS1.3."] pub const DEFAULT_TLS13_PROVIDER : CryptoProvider = CryptoProvider { tls12_cipher_suites : Cow :: Borrowed (& []) , .. DEFAULT_PROVIDER } ;
};
}
