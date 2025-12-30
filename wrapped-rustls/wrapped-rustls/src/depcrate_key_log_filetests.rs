// Generated macro for tests (module)
macro_rules! Depcrate_key_log_filetests {
() => {
// Module: crate::key_log_file
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (test , target_os = "linux"))] mod tests { use super :: * ; fn init () { let _ = env_logger :: builder () . is_test (true) . try_init () ; } # [test] fn test_env_var_is_not_set () { init () ; let mut inner = KeyLogFileInner :: new (None) ; assert ! (inner . try_write ("label" , b"random" , b"secret") . is_ok ()) ; } # [test] fn test_env_var_cannot_be_opened () { init () ; let mut inner = KeyLogFileInner :: new (Some ("/dev/does-not-exist" . into ())) ; assert ! (inner . try_write ("label" , b"random" , b"secret") . is_ok ()) ; } # [test] fn test_env_var_cannot_be_written () { init () ; let mut inner = KeyLogFileInner :: new (Some ("/dev/full" . into ())) ; assert ! (inner . try_write ("label" , b"random" , b"secret") . is_err ()) ; } }
};
}
