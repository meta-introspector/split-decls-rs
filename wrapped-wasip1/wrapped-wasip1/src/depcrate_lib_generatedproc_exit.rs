// Generated macro for proc_exit (function)
macro_rules! Depcrate_lib_generatedproc_exit {
() => {
// Module: crate::lib_generated
// Provides: {"proc_exit"}
// Dependencies: {}
# [doc = " Terminate the process normally. An exit code of 0 indicates successful"] # [doc = " termination of the program. The meanings of other values is dependent on"] # [doc = " the environment."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `rval` - The exit code returned by the process."] pub unsafe fn proc_exit (rval : Exitcode) { wasi_snapshot_preview1 :: proc_exit (rval as i32) ; }
};
}
