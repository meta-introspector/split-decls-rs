// Generated macro for IoResultExt (trait)
macro_rules! Depcrate_errorIoResultExt {
() => {
// Module: crate::error
// Provides: {"IoResultExt"}
// Dependencies: {}
pub (crate) trait IoResultExt < T > { fn with_err_path < F , P > (self , path : F) -> Self where F : FnOnce () -> P , P : Into < PathBuf > ; }
};
}
