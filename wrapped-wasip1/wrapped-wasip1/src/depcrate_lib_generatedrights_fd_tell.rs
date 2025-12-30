// Generated macro for RIGHTS_FD_TELL (const)
macro_rules! Depcrate_lib_generatedRIGHTS_FD_TELL {
() => {
// Module: crate::lib_generated
// Provides: {"RIGHTS_FD_TELL"}
// Dependencies: {}
# [doc = " The right to invoke `fd_seek` in such a way that the file offset"] # [doc = " remains unaltered (i.e., `whence::cur` with offset zero), or to"] # [doc = " invoke `fd_tell`."] pub const RIGHTS_FD_TELL : Rights = 1 << 5 ;
};
}
