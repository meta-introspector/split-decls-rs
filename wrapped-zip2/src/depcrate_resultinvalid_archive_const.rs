// Generated macro for invalid_archive_const (function)
macro_rules! Depcrate_resultinvalid_archive_const {
() => {
// Module: crate::result
// Provides: {"invalid_archive_const"}
// Dependencies: {}
pub (crate) const fn invalid_archive_const (message : & 'static str) -> ZipError { ZipError :: InvalidArchive (Cow :: Borrowed (message)) }
};
}
