// Generated macro for impl_238 (impl)
macro_rules! Depcrate_lib_generatedimpl_238 {
() => {
// Module: crate::lib_generated
// Provides: {"impl_238"}
// Dependencies: {}
impl fmt :: Debug for Signal { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Signal") . field ("code" , & self . 0) . field ("name" , & self . name ()) . field ("message" , & self . message ()) . finish () } }
};
}
