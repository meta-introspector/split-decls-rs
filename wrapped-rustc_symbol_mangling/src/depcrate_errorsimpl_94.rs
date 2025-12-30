// Generated macro for impl_94 (impl)
macro_rules! Depcrate_errorsimpl_94 {
() => {
// Module: crate::errors
// Provides: {"impl_94"}
// Dependencies: {}
impl fmt :: Display for Kind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Kind :: SymbolName => write ! (f , "symbol-name") , Kind :: Demangling => write ! (f , "demangling") , Kind :: DemanglingAlt => write ! (f , "demangling-alt") , Kind :: DefPath => write ! (f , "def-path") , } } }
};
}
