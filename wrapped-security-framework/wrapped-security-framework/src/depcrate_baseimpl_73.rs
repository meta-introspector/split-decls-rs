// Generated macro for impl_73 (impl)
macro_rules! Depcrate_baseimpl_73 {
() => {
// Module: crate::base
// Provides: {"impl_73"}
// Dependencies: {}
impl fmt :: Debug for Error { # [cold] fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut builder = fmt . debug_struct ("Error") ; builder . field ("code" , & self . 0) ; if let Some (message) = self . message () { builder . field ("message" , & message) ; } builder . finish () } }
};
}
