// Generated macro for impl_66 (impl)
macro_rules! Depcrate_errorimpl_66 {
() => {
// Module: crate::error
// Provides: {"impl_66"}
// Dependencies: {}
impl de :: Error for Error { fn custom < T : Display > (msg : T) -> Self { Error { msg : msg . to_string () , } } }
};
}
