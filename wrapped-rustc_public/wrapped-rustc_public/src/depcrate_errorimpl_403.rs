// Generated macro for impl_403 (impl)
macro_rules! Depcrate_errorimpl_403 {
() => {
// Module: crate::error
// Provides: {"impl_403"}
// Dependencies: {}
impl bridge :: Error for Error { fn new (msg : String) -> Self { Self (msg) } fn from_internal < T : Debug > (err : T) -> Self { Self (format ! ("{err:?}")) } }
};
}
