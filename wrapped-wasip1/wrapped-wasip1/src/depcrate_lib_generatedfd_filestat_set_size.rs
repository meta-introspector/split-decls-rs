// Generated macro for fd_filestat_set_size (function)
macro_rules! Depcrate_lib_generatedfd_filestat_set_size {
() => {
// Module: crate::lib_generated
// Provides: {"fd_filestat_set_size"}
// Dependencies: {}
# [doc = " Adjust the size of an open file. If this increases the file's size, the extra bytes are filled with zeros."] # [doc = " Note: This is similar to `ftruncate` in POSIX."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `size` - The desired file size."] pub unsafe fn fd_filestat_set_size (fd : Fd , size : Filesize) -> Result < () , Errno > { let ret = wasi_snapshot_preview1 :: fd_filestat_set_size (fd as i32 , size as i64) ; match ret { 0 => Ok (()) , _ => Err (Errno (ret as u16)) , } }
};
}
