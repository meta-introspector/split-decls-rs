// Generated macro for impl_711 (impl)
macro_rules! Depcrate_util_boxed_syncimpl_711 {
() => {
// Module: crate::util::boxed::sync
// Provides: {"impl_711"}
// Dependencies: {}
impl < T , U , E > BoxService < T , U , E > { # [allow (missing_docs)] pub fn new < S > (inner : S) -> Self where S : Service < T , Response = U , Error = E > + Send + 'static , S :: Future : Send + 'static , { let inner : Box < dyn Service < T , Response = U , Error = E , Future = BoxFuture < U , E > > + Send > = Box :: new (inner . map_future (| f : S :: Future | Box :: pin (f) as _)) ; let inner = SyncWrapper :: new (inner) ; BoxService { inner } } # [doc = " Returns a [`Layer`] for wrapping a [`Service`] in a [`BoxService`]"] # [doc = " middleware."] # [doc = ""] # [doc = " [`Layer`]: crate::Layer"] pub fn layer < S > () -> LayerFn < fn (S) -> Self > where S : Service < T , Response = U , Error = E > + Send + 'static , S :: Future : Send + 'static , { layer_fn (Self :: new) } }
};
}
