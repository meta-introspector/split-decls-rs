// Generated macro for make_client_config_with_kx_groups (function)
macro_rules! Depcratemake_client_config_with_kx_groups {
() => {
// Module: crate
// Provides: {"make_client_config_with_kx_groups"}
// Dependencies: {}
pub fn make_client_config_with_kx_groups (kt : KeyType , kx_groups : Vec < & 'static dyn rustls :: crypto :: SupportedKxGroup > , provider : & CryptoProvider ,) -> ClientConfig { ClientConfig :: builder (CryptoProvider { kx_groups : Cow :: Owned (kx_groups) , .. provider . clone () } . into () ,) . finish (kt) }
};
}
