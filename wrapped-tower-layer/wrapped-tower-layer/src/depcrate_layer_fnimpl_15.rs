// Generated macro for impl_15 (impl)
macro_rules! Depcrate_layer_fnimpl_15 {
() => {
// Module: crate::layer_fn
// Provides: {"impl_15"}
// Dependencies: {}
impl < F > fmt :: Debug for LayerFn < F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("LayerFn") . field ("f" , & format_args ! ("{}" , core :: any :: type_name ::< F > ())) . finish () } }
};
}
