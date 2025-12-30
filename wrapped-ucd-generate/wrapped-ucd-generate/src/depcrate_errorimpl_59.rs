// Generated macro for impl_59 (impl)
macro_rules! Depcrate_errorimpl_59 {
() => {
// Module: crate::error
// Provides: {"impl_59"}
// Dependencies: {}
impl From < fst :: Error > for Error { fn from (err : fst :: Error) -> Error { Error :: Other (err . to_string ()) } }
};
}
