macro_rules! macro_358 {
    () => {
        bitflags :: bitflags ! { # [doc = " `IORING_ASYNC_CANCEL_*` flags for use with [`io_uring_sqe`]."] # [repr (transparent)] # [derive (Default , Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct IoringAsyncCancelFlags : u32 { # [doc = " `IORING_ASYNC_CANCEL_ALL`"] const ALL = sys :: IORING_ASYNC_CANCEL_ALL ; # [doc = " `IORING_ASYNC_CANCEL_FD`"] const FD = sys :: IORING_ASYNC_CANCEL_FD ; # [doc = " `IORING_ASYNC_CANCEL_FD`"] const ANY = sys :: IORING_ASYNC_CANCEL_ANY ; # [doc = " `IORING_ASYNC_CANCEL_FD`"] const FD_FIXED = sys :: IORING_ASYNC_CANCEL_FD_FIXED ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
    };
}

macro_358!()