// Generated macro for CCompiler (struct)
macro_rules! Depcrate_compiler_cCCompiler {
() => {
// Module: crate::compiler::c
// Provides: {"CCompiler"}
// Dependencies: {}
# [doc = " A generic implementation of the `Compiler` trait for C/C++ compilers."] # [derive (Clone)] pub struct CCompiler < I > where I : CCompilerImpl , { executable : PathBuf , executable_digest : String , compiler : I , }
};
}
