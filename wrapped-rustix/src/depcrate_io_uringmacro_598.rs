// Generated macro for macro_598 (macro)
macro_rules! Depcrate_io_uringmacro_598 {
() => {
// Module: crate::io_uring
// Provides: {"macro_598"}
// Dependencies: {}
bitflags :: bitflags ! { # [doc = " `IORING_FSYNC_*` flags for use with [`io_uring_sqe`]."] # [repr (transparent)] # [derive (Default , Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct IoringFsyncFlags : u32 { # [doc = " `IORING_FSYNC_DATASYNC`"] const DATASYNC = sys :: IORING_FSYNC_DATASYNC ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
};
}
