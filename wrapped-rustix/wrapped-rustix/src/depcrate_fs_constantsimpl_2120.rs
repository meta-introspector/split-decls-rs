// Generated macro for impl_2120 (impl)
macro_rules! Depcrate_fs_constantsimpl_2120 {
() => {
// Module: crate::fs::constants
// Provides: {"impl_2120"}
// Dependencies: {}
impl FileType { # [doc = " Returns `true` if this `FileType` is a regular file."] pub fn is_file (self) -> bool { self == Self :: RegularFile } # [doc = " Returns `true` if this `FileType` is a directory."] pub fn is_dir (self) -> bool { self == Self :: Directory } # [doc = " Returns `true` if this `FileType` is a symlink."] pub fn is_symlink (self) -> bool { self == Self :: Symlink } # [doc = " Returns `true` if this `FileType` is a fifo."] # [cfg (not (target_os = "wasi"))] pub fn is_fifo (self) -> bool { self == Self :: Fifo } # [doc = " Returns `true` if this `FileType` is a socket."] # [cfg (not (target_os = "wasi"))] pub fn is_socket (self) -> bool { self == Self :: Socket } # [doc = " Returns `true` if this `FileType` is a character device."] pub fn is_char_device (self) -> bool { self == Self :: CharacterDevice } # [doc = " Returns `true` if this `FileType` is a block device."] pub fn is_block_device (self) -> bool { self == Self :: BlockDevice } }
};
}
