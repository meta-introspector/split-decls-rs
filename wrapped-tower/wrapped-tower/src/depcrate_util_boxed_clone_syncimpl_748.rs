// Generated macro for impl_748 (impl)
macro_rules! Depcrate_util_boxed_clone_syncimpl_748 {
() => {
// Module: crate::util::boxed_clone_sync
// Provides: {"impl_748"}
// Dependencies: {}
impl < T , U , E > BoxCloneSyncService < T , U , E > { # [doc = " Create a new `BoxCloneSyncService`."] pub fn new < S > (inner : S) -> Self where S : Service < T , Response = U , Error = E > + Clone + Send + Sync + 'static , S :: Future : Send + 'static , { let inner = inner . map_future (| f | Box :: pin (f) as _) ; BoxCloneSyncService (Box :: new (inner)) } # [doc = " Returns a [`Layer`] for wrapping a [`Service`] in a [`BoxCloneSyncService`]"] # [doc = " middleware."] # [doc = ""] # [doc = " [`Layer`]: crate::Layer"] pub fn layer < S > () -> LayerFn < fn (S) -> Self > where S : Service < T , Response = U , Error = E > + Clone + Send + Sync + 'static , S :: Future : Send + 'static , { layer_fn (Self :: new) } }
};
}
