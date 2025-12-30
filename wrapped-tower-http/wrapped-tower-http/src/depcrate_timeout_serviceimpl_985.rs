// Generated macro for impl_985 (impl)
macro_rules! Depcrate_timeout_serviceimpl_985 {
() => {
// Module: crate::timeout::service
// Provides: {"impl_985"}
// Dependencies: {}
impl < S > RequestBodyTimeout < S > { # [doc = " Creates a new [`RequestBodyTimeout`]."] pub fn new (service : S , timeout : Duration) -> Self { Self { inner : service , timeout , } } # [doc = " Returns a new [`Layer`] that wraps services with a [`RequestBodyTimeoutLayer`] middleware."] # [doc = ""] # [doc = " [`Layer`]: tower_layer::Layer"] pub fn layer (timeout : Duration) -> RequestBodyTimeoutLayer { RequestBodyTimeoutLayer :: new (timeout) } define_inner_service_accessors ! () ; }
};
}
