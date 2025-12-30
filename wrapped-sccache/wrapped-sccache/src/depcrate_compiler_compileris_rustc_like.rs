// Generated macro for is_rustc_like (function)
macro_rules! Depcrate_compiler_compileris_rustc_like {
() => {
// Module: crate::compiler::compiler
// Provides: {"is_rustc_like"}
// Dependencies: {}
# [doc = " Returns true if the given path looks like a program known to have"] # [doc = " a rustc compatible interface."] fn is_rustc_like < P : AsRef < Path > > (p : P) -> bool { matches ! (p . as_ref () . file_stem () . map (| s | s . to_string_lossy () . to_lowercase ()) . as_deref () , Some ("rustc") | Some ("clippy-driver")) }
};
}
