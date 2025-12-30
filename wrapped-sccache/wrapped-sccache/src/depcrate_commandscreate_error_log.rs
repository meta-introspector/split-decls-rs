// Generated macro for create_error_log (function)
macro_rules! Depcrate_commandscreate_error_log {
() => {
// Module: crate::commands
// Provides: {"create_error_log"}
// Dependencies: {}
# [doc = " Create the log file and return an error if cannot be created"] fn create_error_log () -> Result < File > { trace ! ("Create the log file") ; let name = match env :: var ("SCCACHE_ERROR_LOG") { Ok (filename) if ! filename . is_empty () => filename , _ => { bail ! ("Cannot read variable 'SCCACHE_ERROR_LOG'") ; } } ; let f = match OpenOptions :: new () . create (true) . append (true) . open (& name) { Ok (f) => f , Err (_) => { bail ! ("Cannot open/write log file '{}'" , & name) ; } } ; Ok (f) }
};
}
