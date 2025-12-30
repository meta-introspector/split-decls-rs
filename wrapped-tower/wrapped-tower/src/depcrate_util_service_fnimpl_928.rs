// Generated macro for impl_928 (impl)
macro_rules! Depcrate_util_service_fnimpl_928 {
() => {
// Module: crate::util::service_fn
// Provides: {"impl_928"}
// Dependencies: {}
impl < T > fmt :: Debug for ServiceFn < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ServiceFn") . field ("f" , & format_args ! ("{}" , std :: any :: type_name ::< T > ())) . finish () } }
};
}
