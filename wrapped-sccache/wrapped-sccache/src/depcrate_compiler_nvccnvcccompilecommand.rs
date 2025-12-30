// Generated macro for NvccCompileCommand (struct)
macro_rules! Depcrate_compiler_nvccNvccCompileCommand {
() => {
// Module: crate::compiler::nvcc
// Provides: {"NvccCompileCommand"}
// Dependencies: {}
# [derive (Clone , Debug)] pub struct NvccCompileCommand { pub temp_dir : PathBuf , pub keep_dir : Option < PathBuf > , pub num_parallel : usize , pub executable : PathBuf , pub arguments : Vec < OsString > , pub compilation_flag : OsString , pub env_vars : Vec < (OsString , OsString) > , pub cwd : PathBuf , pub host_compiler : NvccHostCompiler , pub output_file_name : OsString , }
};
}
