// Generated macro for get_addr (function)
macro_rules! Depcrate_commandsget_addr {
() => {
// Module: crate::commands
// Provides: {"get_addr"}
// Dependencies: {}
# [doc = " Get the port on which the server should listen."] fn get_addr () -> crate :: net :: SocketAddr { # [cfg (unix)] if let Ok (addr) = env :: var ("SCCACHE_SERVER_UDS") { if let Ok (uds) = crate :: net :: SocketAddr :: parse_uds (& addr) { return uds ; } } let port = env :: var ("SCCACHE_SERVER_PORT") . ok () . and_then (| s | s . parse () . ok ()) . unwrap_or (DEFAULT_PORT) ; crate :: net :: SocketAddr :: with_port (port) }
};
}
