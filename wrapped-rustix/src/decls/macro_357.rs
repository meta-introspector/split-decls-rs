macro_rules! macro_357 {
    () => {
        bitflags :: bitflags ! { # [doc = " `IORING_URING_CMD_*` flags for use with [`io_uring_sqe`]."] # [repr (transparent)] # [derive (Default , Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct IoringUringCmdFlags : u32 { # [doc = " `IORING_URING_CMD_FIXED`"] const FIXED = sys :: IORING_URING_CMD_FIXED ; # [doc = " `IORING_URING_CMD_MASK`"] const MASK = sys :: IORING_URING_CMD_MASK ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
    };
}

macro_357!()