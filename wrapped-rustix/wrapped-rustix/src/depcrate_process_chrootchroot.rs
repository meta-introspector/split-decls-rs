// Generated macro for chroot (function)
macro_rules! Depcrate_process_chrootchroot {
() => {
// Module: crate::process::chroot
// Provides: {"chroot"}
// Dependencies: {}
# [doc = " `chroot(path)`—Change the process root directory."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/chroot.2.html"] # [cfg (feature = "fs")] # [cfg_attr (docsrs , doc (cfg (feature = "fs")))] # [inline] pub fn chroot < P : path :: Arg > (path : P) -> io :: Result < () > { path . into_with_c_str (backend :: process :: syscalls :: chroot) }
};
}
