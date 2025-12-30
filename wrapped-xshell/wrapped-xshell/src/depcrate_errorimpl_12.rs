// Generated macro for impl_12 (impl)
macro_rules! Depcrate_errorimpl_12 {
() => {
// Module: crate::error
// Provides: {"impl_12"}
// Dependencies: {}
impl From < ErrorKind > for Error { fn from (kind : ErrorKind) -> Error { let kind = Box :: new (kind) ; Error { kind } } }
};
}
