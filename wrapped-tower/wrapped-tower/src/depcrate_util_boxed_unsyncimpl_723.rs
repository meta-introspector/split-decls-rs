// Generated macro for impl_723 (impl)
macro_rules! Depcrate_util_boxed_unsyncimpl_723 {
() => {
// Module: crate::util::boxed::unsync
// Provides: {"impl_723"}
// Dependencies: {}
impl < T , U , E > UnsyncBoxService < T , U , E > { # [allow (missing_docs)] pub fn new < S > (inner : S) -> Self where S : Service < T , Response = U , Error = E > + 'static , S :: Future : 'static , { let inner = Box :: new (UnsyncBoxed { inner }) ; UnsyncBoxService { inner } } # [doc = " Returns a [`Layer`] for wrapping a [`Service`] in an [`UnsyncBoxService`] middleware."] # [doc = ""] # [doc = " [`Layer`]: crate::Layer"] pub fn layer < S > () -> LayerFn < fn (S) -> Self > where S : Service < T , Response = U , Error = E > + 'static , S :: Future : 'static , { layer_fn (Self :: new) } }
};
}
