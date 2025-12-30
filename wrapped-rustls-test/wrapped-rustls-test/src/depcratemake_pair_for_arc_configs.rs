// Generated macro for make_pair_for_arc_configs (function)
macro_rules! Depcratemake_pair_for_arc_configs {
() => {
// Module: crate
// Provides: {"make_pair_for_arc_configs"}
// Dependencies: {}
pub fn make_pair_for_arc_configs (client_config : & Arc < ClientConfig > , server_config : & Arc < ServerConfig > ,) -> (ClientConnection , ServerConnection) { (ClientConnection :: new (client_config . clone () , server_name ("localhost")) . unwrap () , ServerConnection :: new (server_config . clone ()) . unwrap () ,) }
};
}
