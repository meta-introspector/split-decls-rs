// Generated macro for open (function)
macro_rules! Depcrate_fs_absopen {
() => {
// Module: crate::fs::abs
// Provides: {"open"}
// Dependencies: {}
# [doc = " `open(path, oflags, mode)`—Opens a file."] # [doc = ""] # [doc = " POSIX guarantees that `open` will use the lowest unused file descriptor,"] # [doc = " however it is not safe in general to rely on this, as file descriptors may"] # [doc = " be unexpectedly allocated on other threads or in libraries."] # [doc = ""] # [doc = " The `Mode` argument is only significant when creating a file."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/open.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/open.2.html"] # [inline] pub fn open < P : path :: Arg > (path : P , flags : OFlags , mode : Mode) -> io :: Result < OwnedFd > { path . into_with_c_str (| path | backend :: fs :: syscalls :: open (path , flags , mode)) }
};
}
