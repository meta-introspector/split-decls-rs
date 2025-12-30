// Generated macro for impl_678 (impl)
macro_rules! Depcrate_util_boxed_layerimpl_678 {
() => {
// Module: crate::util::boxed::layer
// Provides: {"impl_678"}
// Dependencies: {}
impl < In , T , U , E > BoxLayer < In , T , U , E > { # [doc = " Create a new [`BoxLayer`]."] pub fn new < L > (inner_layer : L) -> Self where L : Layer < In > + Send + Sync + 'static , L :: Service : Service < T , Response = U , Error = E > + Send + 'static , < L :: Service as Service < T > > :: Future : Send + 'static , { let layer = layer_fn (move | inner : In | { let out = inner_layer . layer (inner) ; BoxService :: new (out) }) ; Self { boxed : Arc :: new (layer) , } } }
};
}
