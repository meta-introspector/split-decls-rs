// Generated macro for impl_28 (impl)
macro_rules! Depcrate_errorimpl_28 {
() => {
// Module: crate::error
// Provides: {"impl_28"}
// Dependencies: {}
# [cfg (feature = "pem")] impl From < pem :: Error > for Error { fn from (err : pem :: Error) -> Error { der :: Error :: from (err) . into () } }
};
}
