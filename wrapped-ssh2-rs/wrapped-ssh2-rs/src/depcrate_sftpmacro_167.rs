// Generated macro for macro_167 (macro)
macro_rules! Depcrate_sftpmacro_167 {
() => {
// Module: crate::sftp
// Provides: {"macro_167"}
// Dependencies: {}
bitflags ! { # [doc = " Options that can be used to configure how a file is opened"] # [derive (PartialEq , Eq , PartialOrd , Ord , Hash , Debug , Clone , Copy)] pub struct OpenFlags : c_ulong { # [doc = " Open the file for reading."] const READ = raw :: LIBSSH2_FXF_READ ; # [doc = " Open the file for writing. If both this and `Read` are specified,"] # [doc = " the file is opened for both reading and writing."] const WRITE = raw :: LIBSSH2_FXF_WRITE ; # [doc = " Force all writes to append data at the end of the file."] const APPEND = raw :: LIBSSH2_FXF_APPEND ; # [doc = " If this flag is specified, then a new file will be created if one"] # [doc = " does not already exist (if `Truncate` is specified, the new file"] # [doc = " will be truncated to zero length if it previously exists)."] const CREATE = raw :: LIBSSH2_FXF_CREAT ; # [doc = " Forces an existing file with the same name to be truncated to zero"] # [doc = " length when creating a file by specifying `Create`. Using this flag"] # [doc = " implies the `Create` flag."] const TRUNCATE = raw :: LIBSSH2_FXF_TRUNC | Self :: CREATE . bits () ; # [doc = " Causes the request to fail if the named file already exists. Using"] # [doc = " this flag implies the `Create` flag."] const EXCLUSIVE = raw :: LIBSSH2_FXF_EXCL | Self :: CREATE . bits () ; } }
};
}
