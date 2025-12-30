// Generated macro for make_server_config_with_kx_groups (function)
macro_rules! Depcratemake_server_config_with_kx_groups {
() => {
// Module: crate
// Provides: {"make_server_config_with_kx_groups"}
// Dependencies: {}
pub fn make_server_config_with_kx_groups (kt : KeyType , kx_groups : Vec < & 'static dyn rustls :: crypto :: SupportedKxGroup > , provider : & CryptoProvider ,) -> ServerConfig { ServerConfig :: builder (CryptoProvider { kx_groups : Cow :: Owned (kx_groups) , .. provider . clone () } . into () ,) . finish (kt) }
};
}
