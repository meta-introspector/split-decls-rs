// Generated macro for is_nvidia_cicc (function)
macro_rules! Depcrate_compiler_compileris_nvidia_cicc {
() => {
// Module: crate::compiler::compiler
// Provides: {"is_nvidia_cicc"}
// Dependencies: {}
# [doc = " Returns true if the given path looks like cicc"] fn is_nvidia_cicc < P : AsRef < Path > > (p : P) -> bool { matches ! (p . as_ref () . file_stem () . map (| s | s . to_string_lossy () . to_lowercase ()) . as_deref () , Some ("cicc")) }
};
}
