// Generated macro for make_pair_for_configs (function)
macro_rules! Depcratemake_pair_for_configs {
() => {
// Module: crate
// Provides: {"make_pair_for_configs"}
// Dependencies: {}
pub fn make_pair_for_configs (client_config : ClientConfig , server_config : ServerConfig ,) -> (ClientConnection , ServerConnection) { make_pair_for_arc_configs (& Arc :: new (client_config) , & Arc :: new (server_config)) }
};
}
