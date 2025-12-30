// Generated macro for ServerHooks (struct)
macro_rules! Depcrate_http3_driver_serverServerHooks {
() => {
// Module: crate::http3::driver::server
// Provides: {"ServerHooks"}
// Dependencies: {}
pub struct ServerHooks { # [doc = " Helper to enforce limits and timeouts on an HTTP/3 connection."] settings_enforcer : Http3SettingsEnforcer , # [doc = " Tracks the number of requests that have been handled by this driver."] requests : u64 , # [doc = " Handle to the post-accept timeout entry. If present, the server must"] # [doc = " receive a HEADERS frame before this timeout."] post_accept_timeout : Option < TimeoutKey > , }
};
}
