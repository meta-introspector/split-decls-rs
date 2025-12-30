// Generated macro for error_to_output (function)
macro_rules! Depcrate_compiler_nvccerror_to_output {
() => {
// Module: crate::compiler::nvcc
// Provides: {"error_to_output"}
// Dependencies: {}
fn error_to_output (err : Error) -> process :: Output { match err . downcast :: < ProcessError > () { Ok (ProcessError (out)) => out , Err (err) => process :: Output { status : exit_status (1 as ExitStatusValue) , stdout : vec ! [] , stderr : err . to_string () . into_bytes () , } , } }
};
}
