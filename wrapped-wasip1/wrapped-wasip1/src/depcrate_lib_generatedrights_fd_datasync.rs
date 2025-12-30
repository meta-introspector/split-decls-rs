// Generated macro for RIGHTS_FD_DATASYNC (const)
macro_rules! Depcrate_lib_generatedRIGHTS_FD_DATASYNC {
() => {
// Module: crate::lib_generated
// Provides: {"RIGHTS_FD_DATASYNC"}
// Dependencies: {}
# [doc = " The right to invoke `fd_datasync`."] # [doc = " If `path_open` is set, includes the right to invoke"] # [doc = " `path_open` with `fdflags::dsync`."] pub const RIGHTS_FD_DATASYNC : Rights = 1 << 0 ;
};
}
