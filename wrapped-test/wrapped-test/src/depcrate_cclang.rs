// Generated macro for clang (function)
macro_rules! Depcrate_cclang {
() => {
// Module: crate::c
// Provides: {"clang"}
// Dependencies: {}
fn clang (runner : & Runner < '_ >) -> PathBuf { let target = & runner . opts . c . c_target ; match & runner . opts . c . wasi_sdk_path { Some (path) => path . join (format ! ("bin/{target}-clang")) , None => format ! ("{target}-clang") . into () , } }
};
}
