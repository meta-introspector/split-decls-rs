// Generated macro for impl_442 (impl)
macro_rules! Depcrate_map_response_bodyimpl_442 {
() => {
// Module: crate::map_response_body
// Provides: {"impl_442"}
// Dependencies: {}
impl < S , F > MapResponseBody < S , F > { # [doc = " Create a new [`MapResponseBody`]."] # [doc = ""] # [doc = " `F` is expected to be a function that takes a body and returns another body."] pub fn new (service : S , f : F) -> Self { Self { inner : service , f } } # [doc = " Returns a new [`Layer`] that wraps services with a `MapResponseBodyLayer` middleware."] # [doc = ""] # [doc = " [`Layer`]: tower_layer::Layer"] pub fn layer (f : F) -> MapResponseBodyLayer < F > { MapResponseBodyLayer :: new (f) } define_inner_service_accessors ! () ; }
};
}
