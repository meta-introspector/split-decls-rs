// Generated macro for impl_457 (impl)
macro_rules! Depcrate_map_request_bodyimpl_457 {
() => {
// Module: crate::map_request_body
// Provides: {"impl_457"}
// Dependencies: {}
impl < S , F > MapRequestBody < S , F > { # [doc = " Create a new [`MapRequestBody`]."] # [doc = ""] # [doc = " `F` is expected to be a function that takes a body and returns another body."] pub fn new (service : S , f : F) -> Self { Self { inner : service , f } } # [doc = " Returns a new [`Layer`] that wraps services with a `MapRequestBodyLayer` middleware."] # [doc = ""] # [doc = " [`Layer`]: tower_layer::Layer"] pub fn layer (f : F) -> MapRequestBodyLayer < F > { MapRequestBodyLayer :: new (f) } define_inner_service_accessors ! () ; }
};
}
