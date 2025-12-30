// Generated macro for RIGHTS_PATH_FILESTAT_SET_SIZE (const)
macro_rules! Depcrate_lib_generatedRIGHTS_PATH_FILESTAT_SET_SIZE {
() => {
// Module: crate::lib_generated
// Provides: {"RIGHTS_PATH_FILESTAT_SET_SIZE"}
// Dependencies: {}
# [doc = " The right to change a file's size (there is no `path_filestat_set_size`)."] # [doc = " If `path_open` is set, includes the right to invoke `path_open` with `oflags::trunc`."] pub const RIGHTS_PATH_FILESTAT_SET_SIZE : Rights = 1 << 19 ;
};
}
