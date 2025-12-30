// Generated macro for impl_93 (impl)
macro_rules! Depcrate_lib_generatedimpl_93 {
() => {
// Module: crate::lib_generated
// Provides: {"impl_93"}
// Dependencies: {}
impl fmt :: Debug for Errno { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Errno") . field ("code" , & self . 0) . field ("name" , & self . name ()) . field ("message" , & self . message ()) . finish () } }
};
}
