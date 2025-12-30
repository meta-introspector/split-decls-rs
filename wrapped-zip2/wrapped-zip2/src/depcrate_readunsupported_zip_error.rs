// Generated macro for unsupported_zip_error (function)
macro_rules! Depcrate_readunsupported_zip_error {
() => {
// Module: crate::read
// Provides: {"unsupported_zip_error"}
// Dependencies: {}
const fn unsupported_zip_error < T > (detail : & 'static str) -> ZipResult < T > { Err (ZipError :: UnsupportedArchive (detail)) }
};
}
