// Generated macro for notify_server_startup (function)
macro_rules! Depcrate_servernotify_server_startup {
() => {
// Module: crate::server
// Provides: {"notify_server_startup"}
// Dependencies: {}
# [cfg (windows)] fn notify_server_startup (name : & Option < OsString > , status : ServerStartup) -> Result < () > { use fs :: OpenOptions ; let name = match * name { Some (ref s) => s , None => return Ok (()) , } ; let pipe = OpenOptions :: new () . write (true) . read (true) . open (name) ? ; notify_server_startup_internal (pipe , status) }
};
}
