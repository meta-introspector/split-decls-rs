// Generated macro for impl_176 (impl)
macro_rules! Depcrate_sftpimpl_176 {
() => {
// Module: crate::sftp
// Provides: {"impl_176"}
// Dependencies: {}
impl Drop for File { fn drop (& mut self) { if let Some (file_inner) = self . inner . take () { let sftp_inner = file_inner . sftp . 0 . as_ref () . expect ("We are holding an Arc<SftpInnerDropWrapper>, \
                    so nobody could unset this (set on creation)" ,) ; let sess_inner = sftp_inner . sess . lock () ; let was_blocking = sess_inner . is_blocking () ; sess_inner . set_blocking (true) ; let _close_handle_result = unsafe { raw :: libssh2_sftp_close_handle (file_inner . raw) } ; sess_inner . set_blocking (was_blocking) ; } } }
};
}
