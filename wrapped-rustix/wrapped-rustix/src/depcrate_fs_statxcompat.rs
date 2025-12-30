// Generated macro for compat (module)
macro_rules! Depcrate_fs_statxcompat {
() => {
// Module: crate::fs::statx
// Provides: {"compat"}
// Dependencies: {}
# [cfg (not (feature = "linux_4_11"))] mod compat { use crate :: fd :: BorrowedFd ; use crate :: ffi :: CStr ; use crate :: fs :: { AtFlags , Statx , StatxFlags } ; use crate :: { backend , io } ; use core :: sync :: atomic :: { AtomicU8 , Ordering } ; static STATX_STATE : AtomicU8 = AtomicU8 :: new (0) ; # [inline] pub fn statx (dirfd : BorrowedFd < '_ > , path : & CStr , flags : AtFlags , mask : StatxFlags ,) -> io :: Result < Statx > { match STATX_STATE . load (Ordering :: Relaxed) { 0 => statx_init (dirfd , path , flags , mask) , 1 => Err (io :: Errno :: NOSYS) , _ => backend :: fs :: syscalls :: statx (dirfd , path , flags , mask) , } } # [doc = " The first `statx` call. We don't know if `statx` is available yet."] fn statx_init (dirfd : BorrowedFd < '_ > , path : & CStr , flags : AtFlags , mask : StatxFlags ,) -> io :: Result < Statx > { match backend :: fs :: syscalls :: statx (dirfd , path , flags , mask) { Err (err) => statx_error (err) , result => { STATX_STATE . store (2 , Ordering :: Relaxed) ; result } } } # [doc = " The first `statx` call failed. We can get a variety of error codes"] # [doc = " from seccomp configs or faulty FUSE drivers, so we don't trust"] # [doc = " `ENOSYS` or `EPERM` to tell us whether statx is available."] # [cold] fn statx_error (err : io :: Errno) -> io :: Result < Statx > { if backend :: fs :: syscalls :: is_statx_available () { STATX_STATE . store (2 , Ordering :: Relaxed) ; Err (err) } else { STATX_STATE . store (1 , Ordering :: Relaxed) ; Err (io :: Errno :: NOSYS) } } }
};
}
