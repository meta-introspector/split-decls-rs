// Generated macro for FileStat (struct)
macro_rules! Depcrate_sftpFileStat {
() => {
// Module: crate::sftp
// Provides: {"FileStat"}
// Dependencies: {}
# [doc = " Metadata information about a remote file."] # [doc = ""] # [doc = " Fields are not necessarily all provided"] # [derive (Debug , Clone , Eq , PartialEq)] # [allow (missing_copy_implementations)] pub struct FileStat { # [doc = " File size, in bytes of the file."] pub size : Option < u64 > , # [doc = " Owner ID of the file"] pub uid : Option < u32 > , # [doc = " Owning group of the file"] pub gid : Option < u32 > , # [doc = " Permissions (mode) of the file"] pub perm : Option < u32 > , # [doc = " Last access time of the file"] pub atime : Option < u64 > , # [doc = " Last modification time of the file"] pub mtime : Option < u64 > , }
};
}
