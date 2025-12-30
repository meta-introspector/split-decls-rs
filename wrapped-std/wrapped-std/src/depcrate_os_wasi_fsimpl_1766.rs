// Generated macro for impl_1766 (impl)
macro_rules! Depcrate_os_wasi_fsimpl_1766 {
() => {
// Module: crate::os::wasi::fs
// Provides: {"impl_1766"}
// Dependencies: {}
impl FileTypeExt for fs :: FileType { fn is_block_device (& self) -> bool { self . as_inner () . bits () == wasi :: FILETYPE_BLOCK_DEVICE } fn is_char_device (& self) -> bool { self . as_inner () . bits () == wasi :: FILETYPE_CHARACTER_DEVICE } fn is_socket_dgram (& self) -> bool { self . as_inner () . bits () == wasi :: FILETYPE_SOCKET_DGRAM } fn is_socket_stream (& self) -> bool { self . as_inner () . bits () == wasi :: FILETYPE_SOCKET_STREAM } }
};
}
