// Generated macro for impl_876 (impl)
macro_rules! Depcrate_util_map_futureimpl_876 {
() => {
// Module: crate::util::map_future
// Provides: {"impl_876"}
// Dependencies: {}
impl < F > fmt :: Debug for MapFutureLayer < F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("MapFutureLayer") . field ("f" , & format_args ! ("{}" , std :: any :: type_name ::< F > ())) . finish () } }
};
}
