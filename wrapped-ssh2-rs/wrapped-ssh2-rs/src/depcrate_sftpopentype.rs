// Generated macro for OpenType (enum)
macro_rules! Depcrate_sftpOpenType {
() => {
// Module: crate::sftp
// Provides: {"OpenType"}
// Dependencies: {}
# [doc = " How to open a file handle with libssh2."] # [derive (Copy , Clone)] pub enum OpenType { # [doc = " Specify that a file shoud be opened."] File = raw :: LIBSSH2_SFTP_OPENFILE as isize , # [doc = " Specify that a directory should be opened."] Dir = raw :: LIBSSH2_SFTP_OPENDIR as isize , }
};
}
