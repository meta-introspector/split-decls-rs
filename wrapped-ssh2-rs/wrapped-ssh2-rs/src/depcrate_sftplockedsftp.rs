// Generated macro for LockedSftp (struct)
macro_rules! Depcrate_sftpLockedSftp {
() => {
// Module: crate::sftp
// Provides: {"LockedSftp"}
// Dependencies: {}
struct LockedSftp < 'sftp > { raw : * mut raw :: LIBSSH2_SFTP , sess : MutexGuard < 'sftp , SessionInner > , }
};
}
