// Generated macro for args_get (function)
macro_rules! Depcrate_lib_generatedargs_get {
() => {
// Module: crate::lib_generated
// Provides: {"args_get"}
// Dependencies: {}
# [doc = " Read command-line argument data."] # [doc = " The size of the array should match that returned by `args_sizes_get`."] # [doc = " Each argument is expected to be `\\0` terminated."] pub unsafe fn args_get (argv : * mut * mut u8 , argv_buf : * mut u8) -> Result < () , Errno > { let ret = wasi_snapshot_preview1 :: args_get (argv as i32 , argv_buf as i32) ; match ret { 0 => Ok (()) , _ => Err (Errno (ret as u16)) , } }
};
}
