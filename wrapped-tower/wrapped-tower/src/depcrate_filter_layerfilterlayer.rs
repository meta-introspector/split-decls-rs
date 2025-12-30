// Generated macro for FilterLayer (struct)
macro_rules! Depcrate_filter_layerFilterLayer {
() => {
// Module: crate::filter::layer
// Provides: {"FilterLayer"}
// Dependencies: {}
# [doc = " Conditionally dispatch requests to the inner service based on a synchronous"] # [doc = " [predicate]."] # [doc = ""] # [doc = " This [`Layer`] produces instances of the [`Filter`] service."] # [doc = ""] # [doc = " [predicate]: crate::filter::Predicate"] # [doc = " [`Layer`]: crate::Layer"] # [doc = " [`Filter`]: crate::filter::Filter"] # [derive (Debug , Clone)] pub struct FilterLayer < U > { predicate : U , }
};
}
