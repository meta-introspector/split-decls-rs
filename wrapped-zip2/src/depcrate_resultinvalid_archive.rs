// Generated macro for invalid_archive (function)
macro_rules! Depcrate_resultinvalid_archive {
() => {
// Module: crate::result
// Provides: {"invalid_archive"}
// Dependencies: {}
pub (crate) fn invalid_archive < M : Into < Cow < 'static , str > > > (message : M) -> ZipError { ZipError :: InvalidArchive (message . into ()) }
};
}
