macro_rules! macro_353 {
    () => {
        bitflags :: bitflags ! { # [doc = " `IORING_FSYNC_*` flags for use with [`io_uring_sqe`]."] # [repr (transparent)] # [derive (Default , Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct IoringFsyncFlags : u32 { # [doc = " `IORING_FSYNC_DATASYNC`"] const DATASYNC = sys :: IORING_FSYNC_DATASYNC ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
    };
}

macro_353!();