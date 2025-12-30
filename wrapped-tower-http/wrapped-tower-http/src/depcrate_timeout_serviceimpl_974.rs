// Generated macro for impl_974 (impl)
macro_rules! Depcrate_timeout_serviceimpl_974 {
() => {
// Module: crate::timeout::service
// Provides: {"impl_974"}
// Dependencies: {}
impl TimeoutLayer { # [doc = " Creates a new [`TimeoutLayer`]."] # [doc = ""] # [doc = " By default, it will return a `408 Request Timeout` response if the request does not complete within the specified timeout."] # [doc = " To customize the response status code, use the `with_status_code` method."] # [deprecated (since = "0.6.7" , note = "Use `TimeoutLayer::with_status_code` instead")] pub fn new (timeout : Duration) -> Self { Self :: with_status_code (StatusCode :: REQUEST_TIMEOUT , timeout) } # [doc = " Creates a new [`TimeoutLayer`] with the specified status code for the timeout response."] pub fn with_status_code (status_code : StatusCode , timeout : Duration) -> Self { Self { timeout , status_code , } } }
};
}
