// Generated macro for impl_81 (impl)
macro_rules! Depcrate_errorimpl_81 {
() => {
// Module: crate::error
// Provides: {"impl_81"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "[{}] {}" , self . code , self . msg) } }
};
}
