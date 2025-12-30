// Generated macro for AsyncFilterLayer (struct)
macro_rules! Depcrate_filter_layerAsyncFilterLayer {
() => {
// Module: crate::filter::layer
// Provides: {"AsyncFilterLayer"}
// Dependencies: {}
# [doc = " Conditionally dispatch requests to the inner service based on an asynchronous"] # [doc = " [predicate]."] # [doc = ""] # [doc = " This [`Layer`] produces instances of the [`AsyncFilter`] service."] # [doc = ""] # [doc = " [predicate]: crate::filter::AsyncPredicate"] # [doc = " [`Layer`]: crate::Layer"] # [doc = " [`Filter`]: crate::filter::AsyncFilter"] # [derive (Debug , Clone)] pub struct AsyncFilterLayer < U > { predicate : U , }
};
}
