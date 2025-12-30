// Generated macro for impl_135 (impl)
macro_rules! Depcrate_error_formatimpl_135 {
() => {
// Module: crate::error::format
// Provides: {"impl_135"}
// Dependencies: {}
impl From < io :: Error > for Format { # [inline] fn from (err : io :: Error) -> Self { Self :: StdIo (err) } }
};
}
