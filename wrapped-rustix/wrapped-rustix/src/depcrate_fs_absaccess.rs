// Generated macro for access (function)
macro_rules! Depcrate_fs_absaccess {
() => {
// Module: crate::fs::abs
// Provides: {"access"}
// Dependencies: {}
# [doc = " `access(path, access)`—Tests permissions for a file or directory."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/access.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/access.2.html"] # [cfg (not (any (target_os = "espidf" , target_os = "horizon" , target_os = "vita")))] # [inline] pub fn access < P : path :: Arg > (path : P , access : Access) -> io :: Result < () > { path . into_with_c_str (| path | backend :: fs :: syscalls :: access (path , access)) }
};
}
