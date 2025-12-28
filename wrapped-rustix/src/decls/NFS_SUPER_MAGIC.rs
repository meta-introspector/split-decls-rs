macro_rules! NFS_SUPER_MAGIC {
    () => {
        # [doc = " The filesystem magic number for NFS."] # [doc = ""] # [doc = " See [the `fstatfs` manual page] for more information."] # [doc = ""] # [doc = " [the `fstatfs` manual page]: https://man7.org/linux/man-pages/man2/fstatfs.2.html#DESCRIPTION"] # [cfg (linux_kernel)] pub const NFS_SUPER_MAGIC : FsWord = backend :: c :: NFS_SUPER_MAGIC as FsWord ;
    };
}

NFS_SUPER_MAGIC!()