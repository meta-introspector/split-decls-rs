// Generated macro for readlinkat_raw (function)
macro_rules! Depcrate_fs_atreadlinkat_raw {
() => {
// Module: crate::fs::at
// Provides: {"readlinkat_raw"}
// Dependencies: {}
# [doc = " `readlinkat(fd, path)`—Reads the contents of a symlink, without"] # [doc = " allocating."] # [doc = ""] # [doc = " This is the \"raw\" version which avoids allocating, but which truncates the"] # [doc = " string if it doesn't fit in the provided buffer, and doesn't NUL-terminate"] # [doc = " the string."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/readlinkat.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/readlinkat.2.html"] # [inline] pub fn readlinkat_raw < P : path :: Arg , Fd : AsFd , Buf : Buffer < u8 > > (dirfd : Fd , path : P , mut buf : Buf ,) -> io :: Result < Buf :: Output > { let len = path . into_with_c_str (| path | unsafe { backend :: fs :: syscalls :: readlinkat (dirfd . as_fd () , path , buf . parts_mut ()) }) ? ; unsafe { Ok (buf . assume_init (len)) } }
};
}
