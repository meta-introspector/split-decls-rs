// Generated macro for impl_65 (impl)
macro_rules! Depcrate_errorimpl_65 {
() => {
// Module: crate::error
// Provides: {"impl_65"}
// Dependencies: {}
impl ser :: Error for Error { fn custom < T : Display > (msg : T) -> Self { Error { msg : msg . to_string () , } } }
};
}
