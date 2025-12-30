// Generated macro for impl_60 (impl)
macro_rules! Depcrate_errorimpl_60 {
() => {
// Module: crate::error
// Provides: {"impl_60"}
// Dependencies: {}
impl From < ucd_parse :: Error > for Error { fn from (err : ucd_parse :: Error) -> Error { Error :: Other (err . to_string ()) } }
};
}
