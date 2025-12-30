// Generated macro for impl_120 (impl)
macro_rules! Depcrate_androidimpl_120 {
() => {
// Module: crate::android
// Provides: {"impl_120"}
// Dependencies: {}
impl From < JNIError > for Error { # [track_caller] fn from (cause : JNIError) -> Self { if let JNIError :: JavaException = cause { if let Ok (env) = global () . env () { let _ = env . exception_describe () ; let _ = env . exception_clear () ; } } Self } }
};
}
