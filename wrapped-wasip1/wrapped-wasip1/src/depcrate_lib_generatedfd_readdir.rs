// Generated macro for fd_readdir (function)
macro_rules! Depcrate_lib_generatedfd_readdir {
() => {
// Module: crate::lib_generated
// Provides: {"fd_readdir"}
// Dependencies: {}
# [doc = " Read directory entries from a directory."] # [doc = " When successful, the contents of the output buffer consist of a sequence of"] # [doc = " directory entries. Each directory entry consists of a `dirent` object,"] # [doc = " followed by `dirent::d_namlen` bytes holding the name of the directory"] # [doc = " entry."] # [doc = " This function fills the output buffer as much as possible, potentially"] # [doc = " truncating the last directory entry. This allows the caller to grow its"] # [doc = " read buffer size in case it's too small to fit a single large directory"] # [doc = " entry, or skip the oversized directory entry."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `buf` - The buffer where directory entries are stored"] # [doc = " * `cookie` - The location within the directory to start reading"] # [doc = ""] # [doc = " ## Return"] # [doc = ""] # [doc = " The number of bytes stored in the read buffer. If less than the size of the read buffer, the end of the directory has been reached."] pub unsafe fn fd_readdir (fd : Fd , buf : * mut u8 , buf_len : Size , cookie : Dircookie ,) -> Result < Size , Errno > { let mut rp0 = MaybeUninit :: < Size > :: uninit () ; let ret = wasi_snapshot_preview1 :: fd_readdir (fd as i32 , buf as i32 , buf_len as i32 , cookie as i64 , rp0 . as_mut_ptr () as i32 ,) ; match ret { 0 => Ok (core :: ptr :: read (rp0 . as_mut_ptr () as i32 as * const Size)) , _ => Err (Errno (ret as u16)) , } }
};
}
