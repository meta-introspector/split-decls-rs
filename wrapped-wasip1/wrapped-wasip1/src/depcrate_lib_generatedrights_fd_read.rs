// Generated macro for RIGHTS_FD_READ (const)
macro_rules! Depcrate_lib_generatedRIGHTS_FD_READ {
() => {
// Module: crate::lib_generated
// Provides: {"RIGHTS_FD_READ"}
// Dependencies: {}
# [doc = " The right to invoke `fd_read` and `sock_recv`."] # [doc = " If `rights::fd_seek` is set, includes the right to invoke `fd_pread`."] pub const RIGHTS_FD_READ : Rights = 1 << 1 ;
};
}
