// Generated macro for make_pair (function)
macro_rules! Depcratemake_pair {
() => {
// Module: crate
// Provides: {"make_pair"}
// Dependencies: {}
pub fn make_pair (kt : KeyType , provider : & CryptoProvider) -> (ClientConnection , ServerConnection) { make_pair_for_configs (make_client_config (kt , provider) , make_server_config (kt , provider) ,) }
};
}
