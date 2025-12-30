// Generated macro for openat2 (function)
macro_rules! Depcrate_fs_openat2openat2 {
() => {
// Module: crate::fs::openat2
// Provides: {"openat2"}
// Dependencies: {}
# [doc = " `openat2(dirfd, path, OpenHow { oflags, mode, resolve }, sizeof(OpenHow))`—"] # [doc = " Opens a file with more options."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/openat2.2.html"] # [inline] pub fn openat2 < Fd : AsFd , P : path :: Arg > (dirfd : Fd , path : P , oflags : OFlags , mode : Mode , resolve : ResolveFlags ,) -> io :: Result < OwnedFd > { path . into_with_c_str (| path | { backend :: fs :: syscalls :: openat2 (dirfd . as_fd () , path , oflags , mode , resolve) }) }
};
}
