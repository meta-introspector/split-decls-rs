// Generated macro for get_idle_timeout (function)
macro_rules! Depcrate_serverget_idle_timeout {
() => {
// Module: crate::server
// Provides: {"get_idle_timeout"}
// Dependencies: {}
# [doc = " Get the time the server should idle for before shutting down, in seconds."] fn get_idle_timeout () -> u64 { env :: var ("SCCACHE_IDLE_TIMEOUT") . ok () . and_then (| s | s . parse () . ok ()) . unwrap_or (DEFAULT_IDLE_TIMEOUT) }
};
}
