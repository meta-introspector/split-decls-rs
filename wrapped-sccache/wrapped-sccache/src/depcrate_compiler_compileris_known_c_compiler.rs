// Generated macro for is_known_c_compiler (function)
macro_rules! Depcrate_compiler_compileris_known_c_compiler {
() => {
// Module: crate::compiler::compiler
// Provides: {"is_known_c_compiler"}
// Dependencies: {}
# [doc = " Returns true if the given path looks like a c compiler program"] # [doc = ""] # [doc = " This does not check c compilers, it only report programs that are definitely not rustc"] fn is_known_c_compiler < P : AsRef < Path > > (p : P) -> bool { matches ! (p . as_ref () . file_stem () . map (| s | s . to_string_lossy () . to_lowercase ()) . as_deref () , Some ("cc" | "c++" | "gcc" | "g++" | "clang" | "clang++" | "clang-cl" | "cl" | "nvc" | "nvc++" | "nvcc")) }
};
}
