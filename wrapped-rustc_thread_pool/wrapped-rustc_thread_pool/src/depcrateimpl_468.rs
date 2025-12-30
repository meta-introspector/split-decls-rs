// Generated macro for impl_468 (impl)
macro_rules! Depcrateimpl_468 {
() => {
// Module: crate
// Provides: {"impl_468"}
// Dependencies: {}
impl ThreadPoolBuildError { fn new (kind : ErrorKind) -> ThreadPoolBuildError { ThreadPoolBuildError { kind } } fn is_unsupported (& self) -> bool { matches ! (& self . kind , ErrorKind :: IOError (e) if e . kind () == io :: ErrorKind :: Unsupported) } }
};
}
