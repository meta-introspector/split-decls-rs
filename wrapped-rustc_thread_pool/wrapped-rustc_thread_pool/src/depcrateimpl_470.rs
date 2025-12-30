// Generated macro for impl_470 (impl)
macro_rules! Depcrateimpl_470 {
() => {
// Module: crate
// Provides: {"impl_470"}
// Dependencies: {}
impl fmt :: Display for ThreadPoolBuildError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match & self . kind { ErrorKind :: GlobalPoolAlreadyInitialized => { "The global thread pool has already been initialized." . fmt (f) } ErrorKind :: IOError (e) => e . fmt (f) , } } }
};
}
