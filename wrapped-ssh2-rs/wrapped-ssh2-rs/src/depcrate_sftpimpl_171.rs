// Generated macro for impl_171 (impl)
macro_rules! Depcrate_sftpimpl_171 {
() => {
// Module: crate::sftp
// Provides: {"impl_171"}
// Dependencies: {}
impl Drop for SftpInnerDropWrapper { fn drop (& mut self) { if let Some (inner) = self . 0 . take () { let sess = inner . sess . lock () ; let was_blocking = sess . is_blocking () ; sess . set_blocking (true) ; let _shutdown_result = unsafe { raw :: libssh2_sftp_shutdown (inner . raw) } ; sess . set_blocking (was_blocking) ; } } }
};
}
