// Generated macro for impl_469 (impl)
macro_rules! Depcrateimpl_469 {
() => {
// Module: crate
// Provides: {"impl_469"}
// Dependencies: {}
impl Error for ThreadPoolBuildError { fn source (& self) -> Option < & (dyn Error + 'static) > { match & self . kind { ErrorKind :: GlobalPoolAlreadyInitialized => None , ErrorKind :: IOError (e) => Some (e) , } } }
};
}
