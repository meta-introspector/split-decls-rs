// Generated macro for impl_152 (impl)
macro_rules! Depcrate_lib_generatedimpl_152 {
() => {
// Module: crate::lib_generated
// Provides: {"impl_152"}
// Dependencies: {}
impl Filetype { pub const fn raw (& self) -> u8 { self . 0 } pub fn name (& self) -> & 'static str { match self . 0 { 0 => "UNKNOWN" , 1 => "BLOCK_DEVICE" , 2 => "CHARACTER_DEVICE" , 3 => "DIRECTORY" , 4 => "REGULAR_FILE" , 5 => "SOCKET_DGRAM" , 6 => "SOCKET_STREAM" , 7 => "SYMBOLIC_LINK" , _ => unsafe { core :: hint :: unreachable_unchecked () } , } } pub fn message (& self) -> & 'static str { match self . 0 { 0 => "The type of the file descriptor or file is unknown or is different from any of the other types specified." , 1 => "The file descriptor or file refers to a block device inode." , 2 => "The file descriptor or file refers to a character device inode." , 3 => "The file descriptor or file refers to a directory inode." , 4 => "The file descriptor or file refers to a regular file inode." , 5 => "The file descriptor or file refers to a datagram socket." , 6 => "The file descriptor or file refers to a byte-stream socket." , 7 => "The file refers to a symbolic link inode." , _ => unsafe { core :: hint :: unreachable_unchecked () } , } } }
};
}
