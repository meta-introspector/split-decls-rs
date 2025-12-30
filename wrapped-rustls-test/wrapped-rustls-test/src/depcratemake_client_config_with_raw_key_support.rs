// Generated macro for make_client_config_with_raw_key_support (function)
macro_rules! Depcratemake_client_config_with_raw_key_support {
() => {
// Module: crate
// Provides: {"make_client_config_with_raw_key_support"}
// Dependencies: {}
pub fn make_client_config_with_raw_key_support (kt : KeyType , provider : & CryptoProvider ,) -> ClientConfig { let server_verifier = Arc :: new (MockServerVerifier :: expects_raw_public_keys (provider)) ; let client_cert_resolver = Arc :: new (SingleCredential :: from (kt . certified_client_key (provider) . unwrap () ,)) ; ClientConfig :: builder (provider . clone () . into ()) . dangerous () . with_custom_certificate_verifier (server_verifier) . with_client_credential_resolver (client_cert_resolver) . unwrap () }
};
}
