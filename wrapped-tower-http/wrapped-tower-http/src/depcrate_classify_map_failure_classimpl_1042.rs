// Generated macro for impl_1042 (impl)
macro_rules! Depcrate_classify_map_failure_classimpl_1042 {
() => {
// Module: crate::classify::map_failure_class
// Provides: {"impl_1042"}
// Dependencies: {}
impl < C , F > fmt :: Debug for MapFailureClass < C , F > where C : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("MapFailureClass") . field ("inner" , & self . inner) . field ("f" , & format_args ! ("{}" , std :: any :: type_name ::< F > ())) . finish () } }
};
}
