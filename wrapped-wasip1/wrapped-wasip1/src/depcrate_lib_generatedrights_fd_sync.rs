// Generated macro for RIGHTS_FD_SYNC (const)
macro_rules! Depcrate_lib_generatedRIGHTS_FD_SYNC {
() => {
// Module: crate::lib_generated
// Provides: {"RIGHTS_FD_SYNC"}
// Dependencies: {}
# [doc = " The right to invoke `fd_sync`."] # [doc = " If `path_open` is set, includes the right to invoke"] # [doc = " `path_open` with `fdflags::rsync` and `fdflags::dsync`."] pub const RIGHTS_FD_SYNC : Rights = 1 << 4 ;
};
}
