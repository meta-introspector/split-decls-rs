// Generated macro for impl_6 (impl)
macro_rules! Depcrate_errorimpl_6 {
() => {
// Module: crate::error
// Provides: {"impl_6"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (loc) = self . location { write ! (f , "{}:{}: " , loc . line + 1 , loc . column + 1) ? } write ! (f , "{}" , self . message) } }
};
}
