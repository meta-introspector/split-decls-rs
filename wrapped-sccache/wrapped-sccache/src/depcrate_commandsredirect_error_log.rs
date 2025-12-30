// Generated macro for redirect_error_log (function)
macro_rules! Depcrate_commandsredirect_error_log {
() => {
// Module: crate::commands
// Provides: {"redirect_error_log"}
// Dependencies: {}
# [doc = " If `SCCACHE_ERROR_LOG` is set, redirect stderr to it."] fn redirect_error_log (f : File) -> Result < () > { debug ! ("redirecting stderr into {:?}" , f) ; redirect_stderr (f) ; Ok (()) }
};
}
