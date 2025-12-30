// Generated macro for generate_compile_commands (function)
macro_rules! Depcrate_compiler_tasking_vxgenerate_compile_commands {
() => {
// Module: crate::compiler::tasking_vx
// Provides: {"generate_compile_commands"}
// Dependencies: {}
fn generate_compile_commands (_ : & mut dist :: PathTransformer , executable : & Path , parsed_args : & ParsedArguments , cwd : & Path , env_vars : & [(OsString , OsString)] ,) -> Result < (SingleCompileCommand , Option < dist :: CompileCommand > , Cacheable ,) > { trace ! ("compile") ; let out_file = match parsed_args . outputs . get ("obj") { Some (obj) => obj , None => return Err (anyhow ! ("Missing object file output")) , } ; let mut arguments : Vec < OsString > = vec ! [parsed_args . compilation_flag . clone () , parsed_args . input . clone () . into () , "-o" . into () , out_file . path . as_os_str () . into () ,] ; arguments . extend_from_slice (& parsed_args . preprocessor_args) ; arguments . extend_from_slice (& parsed_args . unhashed_args) ; arguments . extend_from_slice (& parsed_args . common_args) ; let command = SingleCompileCommand { executable : executable . to_owned () , arguments , env_vars : env_vars . to_owned () , cwd : cwd . to_owned () , } ; Ok ((command , None , Cacheable :: Yes)) }
};
}
