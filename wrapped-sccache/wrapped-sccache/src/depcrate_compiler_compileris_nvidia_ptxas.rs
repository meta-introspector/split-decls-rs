// Generated macro for is_nvidia_ptxas (function)
macro_rules! Depcrate_compiler_compileris_nvidia_ptxas {
() => {
// Module: crate::compiler::compiler
// Provides: {"is_nvidia_ptxas"}
// Dependencies: {}
# [doc = " Returns true if the given path looks like ptxas"] fn is_nvidia_ptxas < P : AsRef < Path > > (p : P) -> bool { matches ! (p . as_ref () . file_stem () . map (| s | s . to_string_lossy () . to_lowercase ()) . as_deref () , Some ("ptxas")) }
};
}
