// Generated macro for impl_13 (impl)
macro_rules! Depcrate_lib_generatedimpl_13 {
() => {
// Module: crate::lib_generated
// Provides: {"impl_13"}
// Dependencies: {}
impl fmt :: Debug for Clockid { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Clockid") . field ("code" , & self . 0) . field ("name" , & self . name ()) . field ("message" , & self . message ()) . finish () } }
};
}
