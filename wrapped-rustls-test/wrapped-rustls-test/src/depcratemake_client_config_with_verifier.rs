// Generated macro for make_client_config_with_verifier (function)
macro_rules! Depcratemake_client_config_with_verifier {
() => {
// Module: crate
// Provides: {"make_client_config_with_verifier"}
// Dependencies: {}
pub fn make_client_config_with_verifier (verifier_builder : ServerVerifierBuilder , provider : & CryptoProvider ,) -> ClientConfig { ClientConfig :: builder (provider . clone () . into ()) . dangerous () . with_custom_certificate_verifier (verifier_builder . build () . unwrap ()) . with_no_client_auth () . unwrap () }
};
}
