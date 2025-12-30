// Generated macro for impl_48 (impl)
macro_rules! Depcrate_errorimpl_48 {
() => {
// Module: crate::error
// Provides: {"impl_48"}
// Dependencies: {}
# [cfg (feature = "pem")] impl From < pem :: Error > for Error { fn from (err : pem :: Error) -> Error { der :: Error :: from (err) . into () } }
};
}
