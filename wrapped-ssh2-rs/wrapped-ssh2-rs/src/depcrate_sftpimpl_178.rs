// Generated macro for impl_178 (impl)
macro_rules! Depcrate_sftpimpl_178 {
() => {
// Module: crate::sftp
// Provides: {"impl_178"}
// Dependencies: {}
impl FileType { # [doc = " Test whether this file type represents a directory."] pub fn is_dir (& self) -> bool { self == & FileType :: Directory } # [doc = " Test whether this file type represents a regular file."] pub fn is_file (& self) -> bool { self == & FileType :: RegularFile } # [doc = " Test whether this file type represents a symbolic link."] pub fn is_symlink (& self) -> bool { self == & FileType :: Symlink } fn from_perm (perm : c_ulong) -> Self { match perm & raw :: LIBSSH2_SFTP_S_IFMT { raw :: LIBSSH2_SFTP_S_IFIFO => FileType :: NamedPipe , raw :: LIBSSH2_SFTP_S_IFCHR => FileType :: CharDevice , raw :: LIBSSH2_SFTP_S_IFDIR => FileType :: Directory , raw :: LIBSSH2_SFTP_S_IFBLK => FileType :: BlockDevice , raw :: LIBSSH2_SFTP_S_IFREG => FileType :: RegularFile , raw :: LIBSSH2_SFTP_S_IFLNK => FileType :: Symlink , raw :: LIBSSH2_SFTP_S_IFSOCK => FileType :: Socket , other => FileType :: Other (other) , } } }
};
}
