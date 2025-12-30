// Generated macro for impl_947 (impl)
macro_rules! Depcrate_set_statusimpl_947 {
() => {
// Module: crate::set_status
// Provides: {"impl_947"}
// Dependencies: {}
impl < S > SetStatus < S > { # [doc = " Create a new [`SetStatus`]."] # [doc = ""] # [doc = " The response status code will be `status` regardless of what the inner service returns."] pub fn new (inner : S , status : StatusCode) -> Self { Self { status , inner } } define_inner_service_accessors ! () ; # [doc = " Returns a new [`Layer`] that wraps services with a `SetStatus` middleware."] # [doc = ""] # [doc = " [`Layer`]: tower_layer::Layer"] pub fn layer (status : StatusCode) -> SetStatusLayer { SetStatusLayer :: new (status) } }
};
}
