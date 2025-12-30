// Generated macro for RIGHTS_POLL_FD_READWRITE (const)
macro_rules! Depcrate_lib_generatedRIGHTS_POLL_FD_READWRITE {
() => {
// Module: crate::lib_generated
// Provides: {"RIGHTS_POLL_FD_READWRITE"}
// Dependencies: {}
# [doc = " If `rights::fd_read` is set, includes the right to invoke `poll_oneoff` to subscribe to `eventtype::fd_read`."] # [doc = " If `rights::fd_write` is set, includes the right to invoke `poll_oneoff` to subscribe to `eventtype::fd_write`."] pub const RIGHTS_POLL_FD_READWRITE : Rights = 1 << 27 ;
};
}
