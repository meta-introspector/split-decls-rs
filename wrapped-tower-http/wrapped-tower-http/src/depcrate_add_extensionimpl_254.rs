// Generated macro for impl_254 (impl)
macro_rules! Depcrate_add_extensionimpl_254 {
() => {
// Module: crate::add_extension
// Provides: {"impl_254"}
// Dependencies: {}
impl < S , T > AddExtension < S , T > { # [doc = " Create a new [`AddExtension`]."] pub fn new (inner : S , value : T) -> Self { Self { inner , value } } define_inner_service_accessors ! () ; # [doc = " Returns a new [`Layer`] that wraps services with a `AddExtension` middleware."] # [doc = ""] # [doc = " [`Layer`]: tower_layer::Layer"] pub fn layer (value : T) -> AddExtensionLayer < T > { AddExtensionLayer :: new (value) } }
};
}
