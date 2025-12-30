// Generated macro for proc_raise (function)
macro_rules! Depcrate_lib_generatedproc_raise {
() => {
// Module: crate::lib_generated
// Provides: {"proc_raise"}
// Dependencies: {}
# [doc = " Send a signal to the process of the calling thread."] # [doc = " Note: This is similar to `raise` in POSIX."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `sig` - The signal condition to trigger."] pub unsafe fn proc_raise (sig : Signal) -> Result < () , Errno > { let ret = wasi_snapshot_preview1 :: proc_raise (sig . 0 as i32) ; match ret { 0 => Ok (()) , _ => Err (Errno (ret as u16)) , } }
};
}
