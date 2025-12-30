// Generated macro for make_client_config_with_auth (function)
macro_rules! Depcratemake_client_config_with_auth {
() => {
// Module: crate
// Provides: {"make_client_config_with_auth"}
// Dependencies: {}
pub fn make_client_config_with_auth (kt : KeyType , provider : & CryptoProvider) -> ClientConfig { ClientConfig :: builder (provider . clone () . into ()) . finish_with_creds (kt) }
};
}
