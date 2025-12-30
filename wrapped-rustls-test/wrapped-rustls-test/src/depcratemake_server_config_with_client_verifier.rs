// Generated macro for make_server_config_with_client_verifier (function)
macro_rules! Depcratemake_server_config_with_client_verifier {
() => {
// Module: crate
// Provides: {"make_server_config_with_client_verifier"}
// Dependencies: {}
pub fn make_server_config_with_client_verifier (kt : KeyType , verifier_builder : ClientVerifierBuilder , provider : & CryptoProvider ,) -> ServerConfig { ServerConfig :: builder (provider . clone () . into ()) . with_client_cert_verifier (verifier_builder . build () . unwrap ()) . with_single_cert (kt . identity () , kt . key ()) . unwrap () }
};
}
