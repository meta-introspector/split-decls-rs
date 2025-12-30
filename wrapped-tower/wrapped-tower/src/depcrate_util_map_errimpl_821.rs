// Generated macro for impl_821 (impl)
macro_rules! Depcrate_util_map_errimpl_821 {
() => {
// Module: crate::util::map_err
// Provides: {"impl_821"}
// Dependencies: {}
impl < S , F > MapErr < S , F > { # [doc = " Creates a new [`MapErr`] service."] pub const fn new (inner : S , f : F) -> Self { MapErr { f , inner } } # [doc = " Returns a new [`Layer`] that produces [`MapErr`] services."] # [doc = ""] # [doc = " This is a convenience function that simply calls [`MapErrLayer::new`]."] # [doc = ""] # [doc = " [`Layer`]: tower_layer::Layer"] pub fn layer (f : F) -> MapErrLayer < F > { MapErrLayer { f } } }
};
}
