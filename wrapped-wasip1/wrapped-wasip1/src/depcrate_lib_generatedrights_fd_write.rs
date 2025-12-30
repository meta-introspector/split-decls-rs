// Generated macro for RIGHTS_FD_WRITE (const)
macro_rules! Depcrate_lib_generatedRIGHTS_FD_WRITE {
() => {
// Module: crate::lib_generated
// Provides: {"RIGHTS_FD_WRITE"}
// Dependencies: {}
# [doc = " The right to invoke `fd_write` and `sock_send`."] # [doc = " If `rights::fd_seek` is set, includes the right to invoke `fd_pwrite`."] pub const RIGHTS_FD_WRITE : Rights = 1 << 6 ;
};
}
