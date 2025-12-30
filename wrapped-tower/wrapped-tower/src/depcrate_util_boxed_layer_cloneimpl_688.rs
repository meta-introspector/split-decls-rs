// Generated macro for impl_688 (impl)
macro_rules! Depcrate_util_boxed_layer_cloneimpl_688 {
() => {
// Module: crate::util::boxed::layer_clone
// Provides: {"impl_688"}
// Dependencies: {}
impl < In , T , U , E > BoxCloneServiceLayer < In , T , U , E > { # [doc = " Create a new [`BoxCloneServiceLayer`]."] pub fn new < L > (inner_layer : L) -> Self where L : Layer < In > + Send + Sync + 'static , L :: Service : Service < T , Response = U , Error = E > + Send + Clone + 'static , < L :: Service as Service < T > > :: Future : Send + 'static , { let layer = layer_fn (move | inner : In | { let out = inner_layer . layer (inner) ; BoxCloneService :: new (out) }) ; Self { boxed : Arc :: new (layer) , } } }
};
}
