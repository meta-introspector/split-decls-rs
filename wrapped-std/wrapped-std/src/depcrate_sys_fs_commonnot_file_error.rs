// Generated macro for NOT_FILE_ERROR (const)
macro_rules! Depcrate_sys_fs_commonNOT_FILE_ERROR {
() => {
// Module: crate::sys::fs::common
// Provides: {"NOT_FILE_ERROR"}
// Dependencies: {}
pub (crate) const NOT_FILE_ERROR : Error = io :: const_error ! (ErrorKind :: InvalidInput , "the source path is neither a regular file nor a symlink to a regular file" ,) ;
};
}
