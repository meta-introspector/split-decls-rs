// Generated macro for impl_76 (impl)
macro_rules! Depcrate_baseimpl_76 {
() => {
// Module: crate::base
// Provides: {"impl_76"}
// Dependencies: {}
impl fmt :: Display for Error { # [cold] fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (message) = self . message () { write ! (fmt , "{message}") } else { write ! (fmt , "error code {}" , self . code ()) } } }
};
}
