// Generated macro for CCompilerHasher (struct)
macro_rules! Depcrate_compiler_cCCompilerHasher {
() => {
// Module: crate::compiler::c
// Provides: {"CCompilerHasher"}
// Dependencies: {}
# [doc = " A generic implementation of the `CompilerHasher` trait for C/C++ compilers."] # [derive (Debug , Clone)] pub struct CCompilerHasher < I > where I : CCompilerImpl , { parsed_args : ParsedArguments , executable : PathBuf , executable_digest : String , compiler : I , }
};
}
