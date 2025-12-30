// Generated macro for tls12_suite (function)
macro_rules! Depcrate_cryptotls12_suite {
() => {
// Module: crate::crypto
// Provides: {"tls12_suite"}
// Dependencies: {}
# [cfg (test)] pub (crate) fn tls12_suite (suite : CipherSuite , provider : & CryptoProvider ,) -> & 'static Tls12CipherSuite { provider . tls12_cipher_suites . iter () . find (| cs | cs . common . suite == suite) . unwrap () }
};
}
