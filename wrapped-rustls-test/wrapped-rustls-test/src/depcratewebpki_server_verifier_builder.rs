// Generated macro for webpki_server_verifier_builder (function)
macro_rules! Depcratewebpki_server_verifier_builder {
() => {
// Module: crate
// Provides: {"webpki_server_verifier_builder"}
// Dependencies: {}
pub fn webpki_server_verifier_builder (roots : Arc < RootCertStore > , provider : & CryptoProvider ,) -> ServerVerifierBuilder { WebPkiServerVerifier :: builder (roots , provider) }
};
}
