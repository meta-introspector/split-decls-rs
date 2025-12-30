// Generated macro for is_nvidia_cudafe (function)
macro_rules! Depcrate_compiler_compileris_nvidia_cudafe {
() => {
// Module: crate::compiler::compiler
// Provides: {"is_nvidia_cudafe"}
// Dependencies: {}
# [doc = " Returns true if the given path looks like cudafe++"] fn is_nvidia_cudafe < P : AsRef < Path > > (p : P) -> bool { matches ! (p . as_ref () . file_stem () . map (| s | s . to_string_lossy () . to_lowercase ()) . as_deref () , Some ("cudafe++")) }
};
}
