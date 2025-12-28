macro_rules! macro_356 {
    () => {
        bitflags :: bitflags ! { # [doc = " `IORING_MSG_RING_*` flags for use with [`io_uring_sqe`]."] # [repr (transparent)] # [derive (Default , Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct IoringMsgringFlags : u32 { # [doc = " `IORING_MSG_RING_CQE_SKIP`"] const CQE_SKIP = sys :: IORING_MSG_RING_CQE_SKIP ; # [doc = " `IORING_MSG_RING_FLAGS_PASS`"] const FLAGS_PASS = sys :: IORING_MSG_RING_FLAGS_PASS ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
    };
}

macro_356!();