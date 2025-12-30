// Generated macro for LockedFile (struct)
macro_rules! Depcrate_sftpLockedFile {
() => {
// Module: crate::sftp
// Provides: {"LockedFile"}
// Dependencies: {}
struct LockedFile < 'file > { raw : * mut raw :: LIBSSH2_SFTP_HANDLE , sess : MutexGuard < 'file , SessionInner > , }
};
}
