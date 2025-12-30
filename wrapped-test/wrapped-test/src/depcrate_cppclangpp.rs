// Generated macro for clangpp (function)
macro_rules! Depcrate_cppclangpp {
() => {
// Module: crate::cpp
// Provides: {"clangpp"}
// Dependencies: {}
fn clangpp (runner : & Runner < '_ >) -> PathBuf { match & runner . opts . c . wasi_sdk_path { Some (path) => path . join ("bin/wasm32-wasip2-clang++") , None => "wasm32-wasip2-clang++" . into () , } }
};
}
