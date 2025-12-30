// Generated macro for with_c_str_slow_path (function)
macro_rules! Depcrate_path_argwith_c_str_slow_path {
() => {
// Module: crate::path::arg
// Provides: {"with_c_str_slow_path"}
// Dependencies: {}
# [doc = " The slow path which handles any length. In theory OS's only support up to"] # [doc = " `PATH_MAX`, but we let the OS enforce that."] # [allow (unsafe_code , clippy :: int_plus_one)] # [cold] fn with_c_str_slow_path < T , F > (bytes : & [u8] , f : F) -> io :: Result < T > where F : FnOnce (& CStr) -> io :: Result < T > , { # [cfg (feature = "alloc")] { f (& CString :: new (bytes) . map_err (| _cstr_err | io :: Errno :: INVAL) ?) } # [cfg (not (feature = "alloc"))] { # [cfg (all (libc , not (any (target_os = "espidf" , target_os = "horizon" , target_os = "hurd" , target_os = "vita" , target_os = "wasi"))))] const LARGE_PATH_BUFFER_SIZE : usize = libc :: PATH_MAX as usize ; # [cfg (linux_raw)] const LARGE_PATH_BUFFER_SIZE : usize = linux_raw_sys :: general :: PATH_MAX as usize ; # [cfg (any (target_os = "espidf" , target_os = "horizon" , target_os = "hurd" , target_os = "vita" , target_os = "wasi"))] const LARGE_PATH_BUFFER_SIZE : usize = 4096 as usize ; let mut buf = MaybeUninit :: < [u8 ; LARGE_PATH_BUFFER_SIZE] > :: uninit () ; let buf_ptr = buf . as_mut_ptr () . cast :: < u8 > () ; if bytes . len () + 1 > LARGE_PATH_BUFFER_SIZE { return Err (io :: Errno :: NAMETOOLONG) ; } unsafe { ptr :: copy_nonoverlapping (bytes . as_ptr () , buf_ptr , bytes . len ()) ; buf_ptr . add (bytes . len ()) . write (b'\0') ; } match CStr :: from_bytes_with_nul (unsafe { slice :: from_raw_parts (buf_ptr , bytes . len () + 1) }) { Ok (s) => f (s) , Err (_) => Err (io :: Errno :: INVAL) , } } }
};
}
