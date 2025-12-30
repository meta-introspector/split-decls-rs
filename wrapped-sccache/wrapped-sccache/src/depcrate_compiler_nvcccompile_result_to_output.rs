// Generated macro for compile_result_to_output (function)
macro_rules! Depcrate_compiler_nvcccompile_result_to_output {
() => {
// Module: crate::compiler::nvcc
// Provides: {"compile_result_to_output"}
// Dependencies: {}
fn compile_result_to_output (exe : & Path , res : protocol :: CompileFinished) -> process :: Output { if let Some (signal) = res . signal { return process :: Output { status : exit_status (signal as ExitStatusValue) , stdout : res . stdout , stderr : [format ! ("{} terminated (signal: {})" , exe . file_stem () . unwrap () . to_string_lossy () , signal) . as_bytes () , & res . stderr ,] . concat () , } ; } process :: Output { status : exit_status (res . retcode . unwrap_or (0) as ExitStatusValue) , stdout : res . stdout , stderr : res . stderr , } }
};
}
