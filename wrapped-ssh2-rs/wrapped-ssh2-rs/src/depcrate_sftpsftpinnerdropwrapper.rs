// Generated macro for SftpInnerDropWrapper (struct)
macro_rules! Depcrate_sftpSftpInnerDropWrapper {
() => {
// Module: crate::sftp
// Provides: {"SftpInnerDropWrapper"}
// Dependencies: {}
# [doc = " This contains an Option so that we're able to disable the Drop hook when dropping manually,"] # [doc = " while still dropping all the fields of SftpInner (which we couldn't do with `mem::forget`)"] struct SftpInnerDropWrapper (Option < SftpInner >) ;
};
}
