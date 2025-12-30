// Generated macro for notify_server_startup_internal (function)
macro_rules! Depcrate_servernotify_server_startup_internal {
() => {
// Module: crate::server
// Provides: {"notify_server_startup_internal"}
// Dependencies: {}
fn notify_server_startup_internal < W : Write > (mut w : W , status : ServerStartup) -> Result < () > { util :: write_length_prefixed_bincode (& mut w , status) }
};
}
