// Generated macro for webpki_client_verifier_builder (function)
macro_rules! Depcratewebpki_client_verifier_builder {
() => {
// Module: crate
// Provides: {"webpki_client_verifier_builder"}
// Dependencies: {}
pub fn webpki_client_verifier_builder (roots : Arc < RootCertStore > , provider : & CryptoProvider ,) -> ClientVerifierBuilder { WebPkiClientVerifier :: builder (roots , provider) }
};
}
