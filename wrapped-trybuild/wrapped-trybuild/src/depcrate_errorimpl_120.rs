// Generated macro for impl_120 (impl)
macro_rules! Depcrate_errorimpl_120 {
() => {
// Module: crate::error
// Provides: {"impl_120"}
// Dependencies: {}
impl From < io :: Error > for Error { fn from (err : io :: Error) -> Self { Error :: Io (err) } }
};
}
