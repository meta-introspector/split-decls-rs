// Generated macro for impl_872 (impl)
macro_rules! Depcrate_util_map_futureimpl_872 {
() => {
// Module: crate::util::map_future
// Provides: {"impl_872"}
// Dependencies: {}
impl < S , F > fmt :: Debug for MapFuture < S , F > where S : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("MapFuture") . field ("inner" , & self . inner) . field ("f" , & format_args ! ("{}" , std :: any :: type_name ::< F > ())) . finish () } }
};
}
