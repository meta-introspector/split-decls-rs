// Generated macro for impl_725 (impl)
macro_rules! Depcrate_limit_serviceimpl_725 {
() => {
// Module: crate::limit::service
// Provides: {"impl_725"}
// Dependencies: {}
impl < S > RequestBodyLimit < S > { # [doc = " Create a new `RequestBodyLimit` with the given body length limit."] pub fn new (inner : S , limit : usize) -> Self { Self { inner , limit } } define_inner_service_accessors ! () ; # [doc = " Returns a new [`Layer`] that wraps services with a `RequestBodyLimit` middleware."] # [doc = ""] # [doc = " [`Layer`]: tower_layer::Layer"] pub fn layer (limit : usize) -> RequestBodyLimitLayer { RequestBodyLimitLayer :: new (limit) } }
};
}
