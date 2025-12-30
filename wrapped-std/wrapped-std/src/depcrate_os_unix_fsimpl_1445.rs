// Generated macro for impl_1445 (impl)
macro_rules! Depcrate_os_unix_fsimpl_1445 {
() => {
// Module: crate::os::unix::fs
// Provides: {"impl_1445"}
// Dependencies: {}
# [stable (feature = "file_type_ext" , since = "1.5.0")] impl FileTypeExt for fs :: FileType { fn is_block_device (& self) -> bool { self . as_inner () . is (libc :: S_IFBLK) } fn is_char_device (& self) -> bool { self . as_inner () . is (libc :: S_IFCHR) } fn is_fifo (& self) -> bool { self . as_inner () . is (libc :: S_IFIFO) } fn is_socket (& self) -> bool { self . as_inner () . is (libc :: S_IFSOCK) } }
};
}
