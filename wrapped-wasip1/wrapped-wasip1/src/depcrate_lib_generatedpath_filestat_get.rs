// Generated macro for path_filestat_get (function)
macro_rules! Depcrate_lib_generatedpath_filestat_get {
() => {
// Module: crate::lib_generated
// Provides: {"path_filestat_get"}
// Dependencies: {}
# [doc = " Return the attributes of a file or directory."] # [doc = " Note: This is similar to `stat` in POSIX."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `flags` - Flags determining the method of how the path is resolved."] # [doc = " * `path` - The path of the file or directory to inspect."] # [doc = ""] # [doc = " ## Return"] # [doc = ""] # [doc = " The buffer where the file's attributes are stored."] pub unsafe fn path_filestat_get (fd : Fd , flags : Lookupflags , path : & str) -> Result < Filestat , Errno > { let mut rp0 = MaybeUninit :: < Filestat > :: uninit () ; let ret = wasi_snapshot_preview1 :: path_filestat_get (fd as i32 , flags as i32 , path . as_ptr () as i32 , path . len () as i32 , rp0 . as_mut_ptr () as i32 ,) ; match ret { 0 => Ok (core :: ptr :: read (rp0 . as_mut_ptr () as i32 as * const Filestat)) , _ => Err (Errno (ret as u16)) , } }
};
}
