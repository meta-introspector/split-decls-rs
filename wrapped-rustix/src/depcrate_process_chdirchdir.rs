// Generated macro for chdir (function)
macro_rules! Depcrate_process_chdirchdir {
() => {
// Module: crate::process::chdir
// Provides: {"chdir"}
// Dependencies: {}
# [doc = " `chdir(path)`—Change the current working directory."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/chdir.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/chdir.2.html"] # [inline] # [cfg (feature = "fs")] # [cfg_attr (docsrs , doc (cfg (feature = "fs")))] pub fn chdir < P : path :: Arg > (path : P) -> io :: Result < () > { path . into_with_c_str (backend :: process :: syscalls :: chdir) }
};
}
