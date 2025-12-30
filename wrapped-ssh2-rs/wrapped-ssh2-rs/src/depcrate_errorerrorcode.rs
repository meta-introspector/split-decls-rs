// Generated macro for ErrorCode (enum)
macro_rules! Depcrate_errorErrorCode {
() => {
// Module: crate::error
// Provides: {"ErrorCode"}
// Dependencies: {}
# [doc = " An error code originating from a particular source."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub enum ErrorCode { # [doc = " Codes for errors that originate in libssh2."] # [doc = " Can be one of  `LIBSSH2_ERROR_*` constants."] Session (libc :: c_int) , # [doc = " Codes for errors that originate in the SFTP subsystem."] # [doc = " Can be one of `LIBSSH2_FX_*` constants."] SFTP (libc :: c_int) , }
};
}
