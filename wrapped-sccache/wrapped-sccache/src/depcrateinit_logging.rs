// Generated macro for init_logging (function)
macro_rules! Depcrateinit_logging {
() => {
// Module: crate
// Provides: {"init_logging"}
// Dependencies: {}
fn init_logging () { if env :: var (LOGGING_ENV) . is_ok () { let mut builder = env_logger :: Builder :: from_env (LOGGING_ENV) ; if env :: var ("SCCACHE_LOG_MILLIS") . is_ok () { builder . format_timestamp_millis () ; } match builder . try_init () { Ok (_) => () , Err (e) => panic ! ("Failed to initialize logging: {:?}" , e) , } } }
};
}
