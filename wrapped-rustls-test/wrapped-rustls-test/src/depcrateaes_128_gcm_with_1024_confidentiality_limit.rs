// Generated macro for aes_128_gcm_with_1024_confidentiality_limit (function)
macro_rules! Depcrateaes_128_gcm_with_1024_confidentiality_limit {
() => {
// Module: crate
// Provides: {"aes_128_gcm_with_1024_confidentiality_limit"}
// Dependencies: {}
pub fn aes_128_gcm_with_1024_confidentiality_limit (provider : CryptoProvider ,) -> Arc < CryptoProvider > { const CONFIDENTIALITY_LIMIT : u64 = 1024 ; static TLS13_LIMITED_SUITE : OnceLock < rustls :: Tls13CipherSuite > = OnceLock :: new () ; static TLS12_LIMITED_SUITE : OnceLock < rustls :: Tls12CipherSuite > = OnceLock :: new () ; let tls13_limited = TLS13_LIMITED_SUITE . get_or_init (| | { let tls13 = provider . tls13_cipher_suites . iter () . find (| cs | cs . common . suite == CipherSuite :: TLS13_AES_128_GCM_SHA256) . unwrap () ; rustls :: Tls13CipherSuite { common : rustls :: crypto :: CipherSuiteCommon { confidentiality_limit : CONFIDENTIALITY_LIMIT , .. tls13 . common } , .. * * tls13 } }) ; let tls12_limited = TLS12_LIMITED_SUITE . get_or_init (| | { let tls12 = provider . tls12_cipher_suites . iter () . find (| cs | cs . common . suite == CipherSuite :: TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256) . unwrap () ; rustls :: Tls12CipherSuite { common : rustls :: crypto :: CipherSuiteCommon { confidentiality_limit : CONFIDENTIALITY_LIMIT , .. tls12 . common } , .. * * tls12 } }) ; CryptoProvider { tls12_cipher_suites : Cow :: Owned (vec ! [tls12_limited]) , tls13_cipher_suites : Cow :: Owned (vec ! [tls13_limited]) , .. provider } . into () }
};
}
