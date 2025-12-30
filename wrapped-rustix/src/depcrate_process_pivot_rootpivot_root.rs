// Generated macro for pivot_root (function)
macro_rules! Depcrate_process_pivot_rootpivot_root {
() => {
// Module: crate::process::pivot_root
// Provides: {"pivot_root"}
// Dependencies: {}
# [doc = " `pivot_root(new_root, put_old)`—Change the root mount."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/pivot_root.2.html"] # [cfg (feature = "fs")] # [cfg_attr (docsrs , doc (cfg (feature = "fs")))] # [inline] pub fn pivot_root < P : path :: Arg , Q : path :: Arg > (new_root : P , put_old : Q) -> io :: Result < () > { new_root . into_with_c_str (| new_root | { put_old . into_with_c_str (| put_old | backend :: process :: syscalls :: pivot_root (new_root , put_old)) }) }
};
}
