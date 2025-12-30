// Generated macro for impl_173 (impl)
macro_rules! Depcrate_sftpimpl_173 {
() => {
// Module: crate::sftp
// Provides: {"impl_173"}
// Dependencies: {}
impl Read for File { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let locked = self . lock () ? ; let rc = unsafe { raw :: libssh2_sftp_read (locked . raw , buf . as_mut_ptr () as * mut _ , buf . len () as size_t) } ; if rc < 0 { let rc = rc as libc :: c_int ; if let Some (file_inner) = self . inner . as_ref () { let sftp_inner = file_inner . sftp . 0 . as_ref () . expect ("We are holding an Arc<SftpInnerDropWrapper>, \
                        so nobody could unset this (set on creation)" ,) ; Err (Sftp :: error_code_into_error (locked . sess . raw , sftp_inner . raw , rc) . into ()) } else { Err (Error :: from_errno (ErrorCode :: Session (rc)) . into ()) } } else { Ok (rc as usize) } } }
};
}
