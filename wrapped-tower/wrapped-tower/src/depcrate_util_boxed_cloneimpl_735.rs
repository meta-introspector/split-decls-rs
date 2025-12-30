// Generated macro for impl_735 (impl)
macro_rules! Depcrate_util_boxed_cloneimpl_735 {
() => {
// Module: crate::util::boxed_clone
// Provides: {"impl_735"}
// Dependencies: {}
impl < T , U , E > BoxCloneService < T , U , E > { # [doc = " Create a new `BoxCloneService`."] pub fn new < S > (inner : S) -> Self where S : Service < T , Response = U , Error = E > + Clone + Send + 'static , S :: Future : Send + 'static , { let inner = inner . map_future (| f | Box :: pin (f) as _) ; BoxCloneService (Box :: new (inner)) } # [doc = " Returns a [`Layer`] for wrapping a [`Service`] in a [`BoxCloneService`]"] # [doc = " middleware."] # [doc = ""] # [doc = " [`Layer`]: crate::Layer"] pub fn layer < S > () -> LayerFn < fn (S) -> Self > where S : Service < T , Response = U , Error = E > + Clone + Send + 'static , S :: Future : Send + 'static , { layer_fn (Self :: new) } }
};
}
