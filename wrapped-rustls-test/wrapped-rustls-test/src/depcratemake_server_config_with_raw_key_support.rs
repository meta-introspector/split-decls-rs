// Generated macro for make_server_config_with_raw_key_support (function)
macro_rules! Depcratemake_server_config_with_raw_key_support {
() => {
// Module: crate
// Provides: {"make_server_config_with_raw_key_support"}
// Dependencies: {}
pub fn make_server_config_with_raw_key_support (kt : KeyType , provider : & CryptoProvider ,) -> ServerConfig { let mut client_verifier = MockClientVerifier :: new (| | Ok (PeerVerified :: assertion ()) , kt , provider) ; let server_cert_resolver = Arc :: new (SingleCredential :: from (kt . credentials_with_raw_pub_key (provider) . unwrap () ,)) ; client_verifier . expect_raw_public_keys = true ; ServerConfig :: builder (provider . clone () . into ()) . with_client_cert_verifier (Arc :: new (client_verifier)) . with_server_credential_resolver (server_cert_resolver) . unwrap () }
};
}
