// Generated macro for unsafe_plaintext_crypto_provider (function)
macro_rules! Depcrateunsafe_plaintext_crypto_provider {
() => {
// Module: crate
// Provides: {"unsafe_plaintext_crypto_provider"}
// Dependencies: {}
pub fn unsafe_plaintext_crypto_provider (provider : CryptoProvider) -> Arc < CryptoProvider > { static TLS13_PLAIN_SUITE : OnceLock < rustls :: Tls13CipherSuite > = OnceLock :: new () ; let tls13 = TLS13_PLAIN_SUITE . get_or_init (| | { let tls13 = provider . tls13_cipher_suites . iter () . find (| cs | cs . common . suite == CipherSuite :: TLS13_AES_256_GCM_SHA384) . unwrap () ; rustls :: Tls13CipherSuite { aead_alg : & plaintext :: Aead , common : rustls :: crypto :: CipherSuiteCommon { .. tls13 . common } , .. * * tls13 } }) ; CryptoProvider { tls13_cipher_suites : Cow :: Owned (vec ! [tls13]) , .. provider } . into () }
};
}
