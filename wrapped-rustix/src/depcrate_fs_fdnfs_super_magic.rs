// Generated macro for NFS_SUPER_MAGIC (const)
macro_rules! Depcrate_fs_fdNFS_SUPER_MAGIC {
() => {
// Module: crate::fs::fd
// Provides: {"NFS_SUPER_MAGIC"}
// Dependencies: {}
# [doc = " The filesystem magic number for NFS."] # [doc = ""] # [doc = " See [the `fstatfs` manual page] for more information."] # [doc = ""] # [doc = " [the `fstatfs` manual page]: https://man7.org/linux/man-pages/man2/fstatfs.2.html#DESCRIPTION"] # [cfg (linux_kernel)] pub const NFS_SUPER_MAGIC : FsWord = backend :: c :: NFS_SUPER_MAGIC as FsWord ;
};
}
