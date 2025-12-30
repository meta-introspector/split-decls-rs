// Generated macro for impl_2956 (impl)
macro_rules! Depcrate_processimpl_2956 {
() => {
// Module: crate::process
// Provides: {"impl_2956"}
// Dependencies: {}
# [stable (feature = "std_debug" , since = "1.16.0")] impl fmt :: Debug for Child { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Child") . field ("stdin" , & self . stdin) . field ("stdout" , & self . stdout) . field ("stderr" , & self . stderr) . finish_non_exhaustive () } }
};
}
