// Generated macro for FileTypeExt (trait)
macro_rules! Depcrate_os_wasi_fsFileTypeExt {
() => {
// Module: crate::os::wasi::fs
// Provides: {"FileTypeExt"}
// Dependencies: {}
# [doc = " WASI-specific extensions for [`fs::FileType`]."] # [doc = ""] # [doc = " Adds support for special WASI file types such as block/character devices,"] # [doc = " pipes, and sockets."] pub trait FileTypeExt { # [doc = " Returns `true` if this file type is a block device."] fn is_block_device (& self) -> bool ; # [doc = " Returns `true` if this file type is a character device."] fn is_char_device (& self) -> bool ; # [doc = " Returns `true` if this file type is a socket datagram."] fn is_socket_dgram (& self) -> bool ; # [doc = " Returns `true` if this file type is a socket stream."] fn is_socket_stream (& self) -> bool ; # [doc = " Returns `true` if this file type is any type of socket."] fn is_socket (& self) -> bool { self . is_socket_stream () || self . is_socket_dgram () } }
};
}
