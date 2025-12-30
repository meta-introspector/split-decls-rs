// Generated macro for PROC_SUPER_MAGIC (const)
macro_rules! Depcrate_fs_fdPROC_SUPER_MAGIC {
() => {
// Module: crate::fs::fd
// Provides: {"PROC_SUPER_MAGIC"}
// Dependencies: {}
# [doc = " The filesystem magic number for procfs."] # [doc = ""] # [doc = " See [the `fstatfs` manual page] for more information."] # [doc = ""] # [doc = " [the `fstatfs` manual page]: https://man7.org/linux/man-pages/man2/fstatfs.2.html#DESCRIPTION"] # [cfg (linux_kernel)] pub const PROC_SUPER_MAGIC : FsWord = backend :: c :: PROC_SUPER_MAGIC as FsWord ;
};
}
