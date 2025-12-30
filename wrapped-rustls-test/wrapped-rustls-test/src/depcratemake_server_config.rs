// Generated macro for make_server_config (function)
macro_rules! Depcratemake_server_config {
() => {
// Module: crate
// Provides: {"make_server_config"}
// Dependencies: {}
pub fn make_server_config (kt : KeyType , provider : & CryptoProvider) -> ServerConfig { ServerConfig :: builder (provider . clone () . into ()) . finish (kt) }
};
}
