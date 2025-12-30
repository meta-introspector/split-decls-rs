// Generated macro for impl_163 (impl)
macro_rules! Depcrate_lib_generatedimpl_163 {
() => {
// Module: crate::lib_generated
// Provides: {"impl_163"}
// Dependencies: {}
impl fmt :: Debug for Advice { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Advice") . field ("code" , & self . 0) . field ("name" , & self . name ()) . field ("message" , & self . message ()) . finish () } }
};
}
