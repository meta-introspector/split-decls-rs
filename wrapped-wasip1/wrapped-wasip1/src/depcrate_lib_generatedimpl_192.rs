// Generated macro for impl_192 (impl)
macro_rules! Depcrate_lib_generatedimpl_192 {
() => {
// Module: crate::lib_generated
// Provides: {"impl_192"}
// Dependencies: {}
impl fmt :: Debug for Eventtype { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Eventtype") . field ("code" , & self . 0) . field ("name" , & self . name ()) . field ("message" , & self . message ()) . finish () } }
};
}
