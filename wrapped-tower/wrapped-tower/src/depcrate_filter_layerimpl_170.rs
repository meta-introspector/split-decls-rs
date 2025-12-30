// Generated macro for impl_170 (impl)
macro_rules! Depcrate_filter_layerimpl_170 {
() => {
// Module: crate::filter::layer
// Provides: {"impl_170"}
// Dependencies: {}
impl < U > AsyncFilterLayer < U > { # [doc = " Returns a new layer that produces [`AsyncFilter`] services with the given"] # [doc = " [`AsyncPredicate`]."] # [doc = ""] # [doc = " [`AsyncPredicate`]: crate::filter::AsyncPredicate"] # [doc = " [`Filter`]: crate::filter::Filter"] pub const fn new (predicate : U) -> Self { Self { predicate } } }
};
}
