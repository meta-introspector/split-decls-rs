// Generated macro for impl_410 (impl)
macro_rules! Depcrate_errorimpl_410 {
() => {
// Module: crate::error
// Provides: {"impl_410"}
// Dependencies: {}
impl From < io :: Error > for Error { fn from (value : io :: Error) -> Self { Error (value . to_string ()) } }
};
}
