// Generated macro for impl_1941 (impl)
macro_rules! Depcrate_key_log_fileimpl_1941 {
() => {
// Module: crate::key_log_file
// Provides: {"impl_1941"}
// Dependencies: {}
impl KeyLogFile { # [doc = " Makes a new `KeyLogFile`.  The environment variable is"] # [doc = " inspected and the named file is opened during this call."] pub fn new () -> Self { let var = var_os ("SSLKEYLOGFILE") ; Self (Mutex :: new (KeyLogFileInner :: new (var))) } }
};
}
