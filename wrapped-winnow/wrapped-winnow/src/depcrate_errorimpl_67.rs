// Generated macro for impl_67 (impl)
macro_rules! Depcrate_errorimpl_67 {
() => {
// Module: crate::error
// Provides: {"impl_67"}
// Dependencies: {}
impl < E > fmt :: Display for ErrMode < E > where E : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { ErrMode :: Incomplete (Needed :: Size (u)) => write ! (f , "Parsing requires {u} more data") , ErrMode :: Incomplete (Needed :: Unknown) => write ! (f , "Parsing requires more data") , ErrMode :: Cut (c) => write ! (f , "Parsing Failure: {c:?}") , ErrMode :: Backtrack (c) => write ! (f , "Parsing Error: {c:?}") , } } }
};
}
