// Generated macro for impl_832 (impl)
macro_rules! Depcrate_util_map_requestimpl_832 {
() => {
// Module: crate::util::map_request
// Provides: {"impl_832"}
// Dependencies: {}
impl < S , F > MapRequest < S , F > { # [doc = " Creates a new [`MapRequest`] service."] pub const fn new (inner : S , f : F) -> Self { MapRequest { inner , f } } # [doc = " Returns a new [`Layer`] that produces [`MapRequest`] services."] # [doc = ""] # [doc = " This is a convenience function that simply calls [`MapRequestLayer::new`]."] # [doc = ""] # [doc = " [`Layer`]: tower_layer::Layer"] pub fn layer (f : F) -> MapRequestLayer < F > { MapRequestLayer { f } } }
};
}
