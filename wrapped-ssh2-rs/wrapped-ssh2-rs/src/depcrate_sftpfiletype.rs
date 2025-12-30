// Generated macro for FileType (enum)
macro_rules! Depcrate_sftpFileType {
() => {
// Module: crate::sftp
// Provides: {"FileType"}
// Dependencies: {}
# [doc = " An enum representing a type of file."] # [derive (PartialEq)] pub enum FileType { # [doc = " Named pipe (S_IFIFO)"] NamedPipe , # [doc = " Character device (S_IFCHR)"] CharDevice , # [doc = " Block device (S_IFBLK)"] BlockDevice , # [doc = " Directory (S_IFDIR)"] Directory , # [doc = " Regular file (S_IFREG)"] RegularFile , # [doc = " Symbolic link (S_IFLNK)"] Symlink , # [doc = " Unix domain socket (S_IFSOCK)"] Socket , # [doc = " Other filetype (does not correspond to any of the other ones)"] Other (c_ulong) , }
};
}
