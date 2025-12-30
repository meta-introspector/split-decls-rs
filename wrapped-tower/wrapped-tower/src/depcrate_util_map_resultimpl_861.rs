// Generated macro for impl_861 (impl)
macro_rules! Depcrate_util_map_resultimpl_861 {
() => {
// Module: crate::util::map_result
// Provides: {"impl_861"}
// Dependencies: {}
impl < S , F > MapResult < S , F > { # [doc = " Creates a new [`MapResult`] service."] pub const fn new (inner : S , f : F) -> Self { MapResult { f , inner } } # [doc = " Returns a new [`Layer`] that produces [`MapResult`] services."] # [doc = ""] # [doc = " This is a convenience function that simply calls [`MapResultLayer::new`]."] # [doc = ""] # [doc = " [`Layer`]: tower_layer::Layer"] pub fn layer (f : F) -> MapResultLayer < F > { MapResultLayer { f } } }
};
}
