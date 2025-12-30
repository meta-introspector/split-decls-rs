// Generated macro for impl_25 (impl)
macro_rules! Depcrate_errorimpl_25 {
() => {
// Module: crate::error
// Provides: {"impl_25"}
// Dependencies: {}
impl Error { pub (crate) fn new < E : Into < BoxError > > (err : E) -> Error { Error { inner : err . into () } } }
};
}
