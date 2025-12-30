// Generated macro for connect_or_start_server (function)
macro_rules! Depcrate_commandsconnect_or_start_server {
() => {
// Module: crate::commands
// Provides: {"connect_or_start_server"}
// Dependencies: {}
# [doc = " Attempt to connect to an sccache server listening on `addr`, or start one if no server is running."] fn connect_or_start_server (addr : & crate :: net :: SocketAddr , startup_timeout : Option < Duration > ,) -> Result < ServerConnection > { trace ! ("connect_or_start_server({addr})") ; match connect_to_server (addr) { Ok (server) => Ok (server) , Err (ref e) if (e . kind () == io :: ErrorKind :: ConnectionRefused || e . kind () == io :: ErrorKind :: TimedOut) || (e . kind () == io :: ErrorKind :: NotFound && addr . is_unix_path ()) => { match run_server_process (startup_timeout) ? { ServerStartup :: Ok { addr : actual_addr } => { if addr . to_string () != actual_addr { bail ! ("sccache: Listening on address {actual_addr} instead of {addr}") ; } } ServerStartup :: AddrInUse => { debug ! ("AddrInUse: possible parallel server bootstraps, retrying..") ; } ServerStartup :: TimedOut => bail ! ("Timed out waiting for server startup. Maybe the remote service is unreachable?\nRun with SCCACHE_LOG=debug SCCACHE_NO_DAEMON=1 to get more information") , ServerStartup :: Err { reason } => bail ! ("Server startup failed: {}\nRun with SCCACHE_LOG=debug SCCACHE_NO_DAEMON=1 to get more information" , reason) , } let server = connect_with_retry (addr) ? ; Ok (server) } Err (e) => Err (e . into ()) , } }
};
}
