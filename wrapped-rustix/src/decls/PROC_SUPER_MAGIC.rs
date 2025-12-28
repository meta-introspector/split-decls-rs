macro_rules! PROC_SUPER_MAGIC {
    () => {
        # [doc = " The filesystem magic number for procfs."] # [doc = ""] # [doc = " See [the `fstatfs` manual page] for more information."] # [doc = ""] # [doc = " [the `fstatfs` manual page]: https://man7.org/linux/man-pages/man2/fstatfs.2.html#DESCRIPTION"] # [cfg (linux_kernel)] pub const PROC_SUPER_MAGIC : FsWord = backend :: c :: PROC_SUPER_MAGIC as FsWord ;
    };
}

PROC_SUPER_MAGIC!();