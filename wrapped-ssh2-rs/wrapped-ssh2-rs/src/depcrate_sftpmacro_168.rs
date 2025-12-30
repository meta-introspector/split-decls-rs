// Generated macro for macro_168 (macro)
macro_rules! Depcrate_sftpmacro_168 {
() => {
// Module: crate::sftp
// Provides: {"macro_168"}
// Dependencies: {}
bitflags ! { # [doc = " Options to `Sftp::rename`."] # [derive (PartialEq , Eq , PartialOrd , Ord , Hash , Debug , Clone , Copy)] pub struct RenameFlags : c_long { # [doc = " In a rename operation, overwrite the destination if it already"] # [doc = " exists. If this flag is not present then it is an error if the"] # [doc = " destination already exists."] const OVERWRITE = raw :: LIBSSH2_SFTP_RENAME_OVERWRITE ; # [doc = " Inform the remote that an atomic rename operation is desired if"] # [doc = " available."] const ATOMIC = raw :: LIBSSH2_SFTP_RENAME_ATOMIC ; # [doc = " Inform the remote end that the native system calls for renaming"] # [doc = " should be used."] const NATIVE = raw :: LIBSSH2_SFTP_RENAME_NATIVE ; } }
};
}
