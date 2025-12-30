// Generated macro for ErrorInner (enum)
macro_rules! Depcrate_errorErrorInner {
() => {
// Module: crate::error
// Provides: {"ErrorInner"}
// Dependencies: {}
# [derive (Debug)] enum ErrorInner { Io { path : Option < PathBuf > , err : io :: Error } , Loop { ancestor : PathBuf , child : PathBuf } , }
};
}
