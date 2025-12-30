// Generated macro for make_client_config (function)
macro_rules! Depcratemake_client_config {
() => {
// Module: crate
// Provides: {"make_client_config"}
// Dependencies: {}
pub fn make_client_config (kt : KeyType , provider : & CryptoProvider) -> ClientConfig { ClientConfig :: builder (provider . clone () . into ()) . finish (kt) }
};
}
