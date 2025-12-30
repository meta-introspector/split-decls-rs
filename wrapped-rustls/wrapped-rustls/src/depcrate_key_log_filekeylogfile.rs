// Generated macro for KeyLogFile (struct)
macro_rules! Depcrate_key_log_fileKeyLogFile {
() => {
// Module: crate::key_log_file
// Provides: {"KeyLogFile"}
// Dependencies: {}
# [doc = " [`KeyLog`] implementation that opens a file whose name is"] # [doc = " given by the `SSLKEYLOGFILE` environment variable, and writes"] # [doc = " keys into it."] # [doc = ""] # [doc = " If `SSLKEYLOGFILE` is not set, this does nothing."] # [doc = ""] # [doc = " If such a file cannot be opened, or cannot be written then"] # [doc = " this does nothing but logs errors at warning-level."] pub struct KeyLogFile (Mutex < KeyLogFileInner >) ;
};
}
