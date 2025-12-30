// Generated macro for impl_553 (impl)
macro_rules! Depcrate_compiler_compilerimpl_553 {
() => {
// Module: crate::compiler::compiler
// Provides: {"impl_553"}
// Dependencies: {}
impl CompilerKind { pub fn lang_kind (& self , lang : & Language) -> String { match lang { Language :: C | Language :: CHeader | Language :: Cxx | Language :: CxxHeader | Language :: GenericHeader | Language :: ObjectiveC | Language :: ObjectiveCxx | Language :: ObjectiveCxxHeader => "C/C++" , Language :: Cuda => "CUDA" , Language :: CudaFE => "CUDA (Device code)" , Language :: Ptx => "PTX" , Language :: Cubin => "CUBIN" , Language :: Rust => "Rust" , Language :: Hip => "HIP" , } . to_string () } pub fn lang_comp_kind (& self , lang : & Language) -> String { let textual_lang = lang . as_str () . to_owned () ; match self { CompilerKind :: C (CCompilerKind :: Clang) => textual_lang + " [clang]" , CompilerKind :: C (CCompilerKind :: Diab) => textual_lang + " [diab]" , CompilerKind :: C (CCompilerKind :: Gcc) => textual_lang + " [gcc]" , CompilerKind :: C (CCompilerKind :: Msvc) => textual_lang + " [msvc]" , CompilerKind :: C (CCompilerKind :: Nvcc) => textual_lang + " [nvcc]" , CompilerKind :: C (CCompilerKind :: CudaFE) => textual_lang + " [cudafe++]" , CompilerKind :: C (CCompilerKind :: Cicc) => textual_lang + " [cicc]" , CompilerKind :: C (CCompilerKind :: Ptxas) => textual_lang + " [ptxas]" , CompilerKind :: C (CCompilerKind :: Nvhpc) => textual_lang + " [nvhpc]" , CompilerKind :: C (CCompilerKind :: TaskingVX) => textual_lang + " [taskingvx]" , CompilerKind :: Rust => textual_lang , } } }
};
}
