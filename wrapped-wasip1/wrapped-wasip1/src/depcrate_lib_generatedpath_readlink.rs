// Generated macro for path_readlink (function)
macro_rules! Depcrate_lib_generatedpath_readlink {
() => {
// Module: crate::lib_generated
// Provides: {"path_readlink"}
// Dependencies: {}
# [doc = " Read the contents of a symbolic link."] # [doc = " Note: This is similar to `readlinkat` in POSIX."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `path` - The path of the symbolic link from which to read."] # [doc = " * `buf` - The buffer to which to write the contents of the symbolic link."] # [doc = ""] # [doc = " ## Return"] # [doc = ""] # [doc = " The number of bytes placed in the buffer."] pub unsafe fn path_readlink (fd : Fd , path : & str , buf : * mut u8 , buf_len : Size ,) -> Result < Size , Errno > { let mut rp0 = MaybeUninit :: < Size > :: uninit () ; let ret = wasi_snapshot_preview1 :: path_readlink (fd as i32 , path . as_ptr () as i32 , path . len () as i32 , buf as i32 , buf_len as i32 , rp0 . as_mut_ptr () as i32 ,) ; match ret { 0 => Ok (core :: ptr :: read (rp0 . as_mut_ptr () as i32 as * const Size)) , _ => Err (Errno (ret as u16)) , } }
};
}
