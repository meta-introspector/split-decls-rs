// Generated macro for CCompilation (struct)
macro_rules! Depcrate_compiler_cCCompilation {
() => {
// Module: crate::compiler::c
// Provides: {"CCompilation"}
// Dependencies: {}
# [doc = " A generic implementation of the `Compilation` trait for C/C++ compilers."] struct CCompilation < I : CCompilerImpl > { parsed_args : ParsedArguments , is_locally_preprocessed : bool , # [cfg (feature = "dist-client")] preprocessed_input : Vec < u8 > , executable : PathBuf , compiler : I , cwd : PathBuf , env_vars : Vec < (OsString , OsString) > , }
};
}
