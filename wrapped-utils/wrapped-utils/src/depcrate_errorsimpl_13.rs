// Generated macro for impl_13 (impl)
macro_rules! Depcrate_errorsimpl_13 {
() => {
// Module: crate::errors
// Provides: {"impl_13"}
// Dependencies: {}
impl fmt :: Debug for JsError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("JsError") . field ("name" , & self . name) . field ("message" , & self . message) . finish () } }
};
}
