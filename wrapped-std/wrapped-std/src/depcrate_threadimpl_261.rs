// Generated macro for impl_261 (impl)
macro_rules! Depcrate_threadimpl_261 {
() => {
// Module: crate::thread
// Provides: {"impl_261"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl fmt :: Debug for Thread { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Thread") . field ("id" , & self . id ()) . field ("name" , & self . name ()) . finish_non_exhaustive () } }
};
}
