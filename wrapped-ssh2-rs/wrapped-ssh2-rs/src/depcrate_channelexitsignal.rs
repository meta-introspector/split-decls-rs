// Generated macro for ExitSignal (struct)
macro_rules! Depcrate_channelExitSignal {
() => {
// Module: crate::channel
// Provides: {"ExitSignal"}
// Dependencies: {}
# [doc = " Data received from when a program exits with a signal."] pub struct ExitSignal { # [doc = " The exit signal received, if the program did not exit cleanly. Does not"] # [doc = " contain a SIG prefix"] pub exit_signal : Option < String > , # [doc = " Error message provided by the remote server (if any)"] pub error_message : Option < String > , # [doc = " Language tag provided by the remote server (if any)"] pub lang_tag : Option < String > , }
};
}
