// Generated macro for impl_847 (impl)
macro_rules! Depcrate_util_map_responseimpl_847 {
() => {
// Module: crate::util::map_response
// Provides: {"impl_847"}
// Dependencies: {}
impl < S , F > MapResponse < S , F > { # [doc = " Creates a new `MapResponse` service."] pub const fn new (inner : S , f : F) -> Self { MapResponse { f , inner } } # [doc = " Returns a new [`Layer`] that produces [`MapResponse`] services."] # [doc = ""] # [doc = " This is a convenience function that simply calls [`MapResponseLayer::new`]."] # [doc = ""] # [doc = " [`Layer`]: tower_layer::Layer"] pub fn layer (f : F) -> MapResponseLayer < F > { MapResponseLayer { f } } }
};
}
