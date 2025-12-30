// Generated macro for impl_168 (impl)
macro_rules! Depcrate_filter_layerimpl_168 {
() => {
// Module: crate::filter::layer
// Provides: {"impl_168"}
// Dependencies: {}
impl < U > FilterLayer < U > { # [doc = " Returns a new layer that produces [`Filter`] services with the given"] # [doc = " [`Predicate`]."] # [doc = ""] # [doc = " [`Predicate`]: crate::filter::Predicate"] # [doc = " [`Filter`]: crate::filter::Filter"] pub const fn new (predicate : U) -> Self { Self { predicate } } }
};
}
