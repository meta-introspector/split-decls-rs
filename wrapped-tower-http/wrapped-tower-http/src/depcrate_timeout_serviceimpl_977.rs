// Generated macro for impl_977 (impl)
macro_rules! Depcrate_timeout_serviceimpl_977 {
() => {
// Module: crate::timeout::service
// Provides: {"impl_977"}
// Dependencies: {}
impl < S > Timeout < S > { # [doc = " Creates a new [`Timeout`]."] # [doc = ""] # [doc = " By default, it will return a `408 Request Timeout` response if the request does not complete within the specified timeout."] # [doc = " To customize the response status code, use the `with_status_code` method."] # [deprecated (since = "0.6.7" , note = "Use `Timeout::with_status_code` instead")] pub fn new (inner : S , timeout : Duration) -> Self { Self :: with_status_code (inner , StatusCode :: REQUEST_TIMEOUT , timeout) } # [doc = " Creates a new [`Timeout`] with the specified status code for the timeout response."] pub fn with_status_code (inner : S , status_code : StatusCode , timeout : Duration) -> Self { Self { inner , timeout , status_code , } } define_inner_service_accessors ! () ; # [doc = " Returns a new [`Layer`] that wraps services with a `Timeout` middleware."] # [doc = ""] # [doc = " [`Layer`]: tower_layer::Layer"] # [deprecated (since = "0.6.7" , note = "Use `Timeout::layer_with_status_code` instead")] pub fn layer (timeout : Duration) -> TimeoutLayer { TimeoutLayer :: with_status_code (StatusCode :: REQUEST_TIMEOUT , timeout) } # [doc = " Returns a new [`Layer`] that wraps services with a `Timeout` middleware with the specified status code."] pub fn layer_with_status_code (status_code : StatusCode , timeout : Duration) -> TimeoutLayer { TimeoutLayer :: with_status_code (status_code , timeout) } }
};
}
