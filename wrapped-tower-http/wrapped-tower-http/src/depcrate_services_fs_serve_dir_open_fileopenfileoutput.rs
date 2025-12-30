// Generated macro for OpenFileOutput (enum)
macro_rules! Depcrate_services_fs_serve_dir_open_fileOpenFileOutput {
() => {
// Module: crate::services::fs::serve_dir::open_file
// Provides: {"OpenFileOutput"}
// Dependencies: {}
pub (super) enum OpenFileOutput { FileOpened (Box < FileOpened >) , Redirect { location : HeaderValue } , FileNotFound , PreconditionFailed , NotModified , InvalidRedirectUri , InvalidFilename , }
};
}
