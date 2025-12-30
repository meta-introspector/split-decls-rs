// Generated macro for impl_938 (impl)
macro_rules! Depcrate_util_thenimpl_938 {
() => {
// Module: crate::util::then
// Provides: {"impl_938"}
// Dependencies: {}
impl < S , F > Then < S , F > { # [doc = " Creates a new `Then` service."] pub const fn new (inner : S , f : F) -> Self { Then { f , inner } } # [doc = " Returns a new [`Layer`] that produces [`Then`] services."] # [doc = ""] # [doc = " This is a convenience function that simply calls [`ThenLayer::new`]."] # [doc = ""] # [doc = " [`Layer`]: tower_layer::Layer"] pub fn layer (f : F) -> ThenLayer < F > { ThenLayer { f } } }
};
}
