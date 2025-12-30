// Generated macro for impl_870 (impl)
macro_rules! Depcrate_util_map_futureimpl_870 {
() => {
// Module: crate::util::map_future
// Provides: {"impl_870"}
// Dependencies: {}
impl < S , F > MapFuture < S , F > { # [doc = " Creates a new [`MapFuture`] service."] pub const fn new (inner : S , f : F) -> Self { Self { inner , f } } # [doc = " Returns a new [`Layer`] that produces [`MapFuture`] services."] # [doc = ""] # [doc = " This is a convenience function that simply calls [`MapFutureLayer::new`]."] # [doc = ""] # [doc = " [`Layer`]: tower_layer::Layer"] pub fn layer (f : F) -> MapFutureLayer < F > { MapFutureLayer :: new (f) } # [doc = " Get a reference to the inner service"] pub fn get_ref (& self) -> & S { & self . inner } # [doc = " Get a mutable reference to the inner service"] pub fn get_mut (& mut self) -> & mut S { & mut self . inner } # [doc = " Consume `self`, returning the inner service"] pub fn into_inner (self) -> S { self . inner } }
};
}
