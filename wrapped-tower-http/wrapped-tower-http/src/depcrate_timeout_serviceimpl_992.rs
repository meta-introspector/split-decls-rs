// Generated macro for impl_992 (impl)
macro_rules! Depcrate_timeout_serviceimpl_992 {
() => {
// Module: crate::timeout::service
// Provides: {"impl_992"}
// Dependencies: {}
impl < S > ResponseBodyTimeout < S > { # [doc = " Creates a new [`ResponseBodyTimeout`]."] pub fn new (service : S , timeout : Duration) -> Self { Self { inner : service , timeout , } } # [doc = " Returns a new [`Layer`] that wraps services with a [`ResponseBodyTimeoutLayer`] middleware."] # [doc = ""] # [doc = " [`Layer`]: tower_layer::Layer"] pub fn layer (timeout : Duration) -> ResponseBodyTimeoutLayer { ResponseBodyTimeoutLayer :: new (timeout) } define_inner_service_accessors ! () ; }
};
}
