// Generated macro for make_server_config_with_mandatory_client_auth (function)
macro_rules! Depcratemake_server_config_with_mandatory_client_auth {
() => {
// Module: crate
// Provides: {"make_server_config_with_mandatory_client_auth"}
// Dependencies: {}
pub fn make_server_config_with_mandatory_client_auth (kt : KeyType , provider : & CryptoProvider ,) -> ServerConfig { make_server_config_with_client_verifier (kt , webpki_client_verifier_builder (kt . client_root_store () , provider) , provider ,) }
};
}
