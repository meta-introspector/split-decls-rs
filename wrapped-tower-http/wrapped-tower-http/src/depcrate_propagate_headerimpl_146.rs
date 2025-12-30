// Generated macro for impl_146 (impl)
macro_rules! Depcrate_propagate_headerimpl_146 {
() => {
// Module: crate::propagate_header
// Provides: {"impl_146"}
// Dependencies: {}
impl < S > PropagateHeader < S > { # [doc = " Create a new [`PropagateHeader`] that propagates the given header."] pub fn new (inner : S , header : HeaderName) -> Self { Self { inner , header } } define_inner_service_accessors ! () ; # [doc = " Returns a new [`Layer`] that wraps services with a `PropagateHeader` middleware."] # [doc = ""] # [doc = " [`Layer`]: tower_layer::Layer"] pub fn layer (header : HeaderName) -> PropagateHeaderLayer { PropagateHeaderLayer :: new (header) } }
};
}
