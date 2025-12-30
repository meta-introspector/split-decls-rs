// Generated macro for tls13_suite (function)
macro_rules! Depcrate_cryptotls13_suite {
() => {
// Module: crate::crypto
// Provides: {"tls13_suite"}
// Dependencies: {}
# [cfg (test)] pub (crate) fn tls13_suite (suite : CipherSuite , provider : & CryptoProvider ,) -> & 'static Tls13CipherSuite { provider . tls13_cipher_suites . iter () . find (| cs | cs . common . suite == suite) . unwrap () }
};
}
