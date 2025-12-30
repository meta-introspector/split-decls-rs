// Generated macro for provider_with_suites (function)
macro_rules! Depcrateprovider_with_suites {
() => {
// Module: crate
// Provides: {"provider_with_suites"}
// Dependencies: {}
pub fn provider_with_suites (provider : & CryptoProvider , suites : & [SupportedCipherSuite] ,) -> CryptoProvider { let mut tls12_cipher_suites = vec ! [] ; let mut tls13_cipher_suites = vec ! [] ; for suite in suites { match suite { SupportedCipherSuite :: Tls12 (suite) => { tls12_cipher_suites . push (* suite) ; } SupportedCipherSuite :: Tls13 (suite) => { tls13_cipher_suites . push (* suite) ; } _ => unreachable ! () , } } CryptoProvider { tls12_cipher_suites : Cow :: Owned (tls12_cipher_suites) , tls13_cipher_suites : Cow :: Owned (tls13_cipher_suites) , .. provider . clone () } }
};
}
