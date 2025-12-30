// Generated macro for random_get (function)
macro_rules! Depcrate_lib_generatedrandom_get {
() => {
// Module: crate::lib_generated
// Provides: {"random_get"}
// Dependencies: {}
# [doc = " Write high-quality random data into a buffer."] # [doc = " This function blocks when the implementation is unable to immediately"] # [doc = " provide sufficient high-quality random data."] # [doc = " This function may execute slowly, so when large mounts of random data are"] # [doc = " required, it's advisable to use this function to seed a pseudo-random"] # [doc = " number generator, rather than to provide the random data directly."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `buf` - The buffer to fill with random data."] pub unsafe fn random_get (buf : * mut u8 , buf_len : Size) -> Result < () , Errno > { let ret = wasi_snapshot_preview1 :: random_get (buf as i32 , buf_len as i32) ; match ret { 0 => Ok (()) , _ => Err (Errno (ret as u16)) , } }
};
}
