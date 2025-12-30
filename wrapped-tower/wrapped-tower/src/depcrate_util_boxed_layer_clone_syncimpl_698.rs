// Generated macro for impl_698 (impl)
macro_rules! Depcrate_util_boxed_layer_clone_syncimpl_698 {
() => {
// Module: crate::util::boxed::layer_clone_sync
// Provides: {"impl_698"}
// Dependencies: {}
impl < In , T , U , E > BoxCloneSyncServiceLayer < In , T , U , E > { # [doc = " Create a new [`BoxCloneSyncServiceLayer`]."] pub fn new < L > (inner_layer : L) -> Self where L : Layer < In > + Send + Sync + 'static , L :: Service : Service < T , Response = U , Error = E > + Send + Sync + Clone + 'static , < L :: Service as Service < T > > :: Future : Send + 'static , { let layer = layer_fn (move | inner : In | { let out = inner_layer . layer (inner) ; BoxCloneSyncService :: new (out) }) ; Self { boxed : Arc :: new (layer) , } } }
};
}
