// Generated macro for ignore_all_server_io_errors (function)
macro_rules! Depcrate_commandsignore_all_server_io_errors {
() => {
// Module: crate::commands
// Provides: {"ignore_all_server_io_errors"}
// Dependencies: {}
# [doc = " Check if ignoring all response errors"] fn ignore_all_server_io_errors () -> bool { match env :: var ("SCCACHE_IGNORE_SERVER_IO_ERROR") { Ok (ignore_server_error) => ignore_server_error == "1" , Err (_) => false , } }
};
}
