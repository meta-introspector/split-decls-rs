// Generated macro for impl_325 (impl)
macro_rules! Depcrate_http3_settingsimpl_325 {
() => {
// Module: crate::http3::settings
// Provides: {"impl_325"}
// Dependencies: {}
impl From < & Http3Settings > for Http3SettingsEnforcer { fn from (value : & Http3Settings) -> Self { Self { limits : Http3Limits { max_requests_per_connection : value . max_requests_per_connection , } , timeouts : Http3Timeouts { post_accept_timeout : value . post_accept_timeout , delay_queue : DelayQueue :: new () , } , } } }
};
}
