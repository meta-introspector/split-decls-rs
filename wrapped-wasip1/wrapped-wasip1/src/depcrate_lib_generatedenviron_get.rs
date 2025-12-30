// Generated macro for environ_get (function)
macro_rules! Depcrate_lib_generatedenviron_get {
() => {
// Module: crate::lib_generated
// Provides: {"environ_get"}
// Dependencies: {}
# [doc = " Read environment variable data."] # [doc = " The sizes of the buffers should match that returned by `environ_sizes_get`."] # [doc = " Key/value pairs are expected to be joined with `=`s, and terminated with `\\0`s."] pub unsafe fn environ_get (environ : * mut * mut u8 , environ_buf : * mut u8) -> Result < () , Errno > { let ret = wasi_snapshot_preview1 :: environ_get (environ as i32 , environ_buf as i32) ; match ret { 0 => Ok (()) , _ => Err (Errno (ret as u16)) , } }
};
}
