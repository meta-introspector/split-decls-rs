// Generated macro for impl_667 (impl)
macro_rules! Depcrate_util_and_thenimpl_667 {
() => {
// Module: crate::util::and_then
// Provides: {"impl_667"}
// Dependencies: {}
impl < S , F > AndThen < S , F > { # [doc = " Creates a new `AndThen` service."] pub const fn new (inner : S , f : F) -> Self { AndThen { f , inner } } # [doc = " Returns a new [`Layer`] that produces [`AndThen`] services."] # [doc = ""] # [doc = " This is a convenience function that simply calls [`AndThenLayer::new`]."] # [doc = ""] # [doc = " [`Layer`]: tower_layer::Layer"] pub fn layer (f : F) -> AndThenLayer < F > { AndThenLayer { f } } }
};
}
