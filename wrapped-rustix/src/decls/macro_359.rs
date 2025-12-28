macro_rules! macro_359 {
    () => {
        bitflags :: bitflags ! { # [doc = " `IORING_FIXED_FD_*` flags for use with [`io_uring_sqe`]."] # [repr (transparent)] # [derive (Default , Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct IoringFixedFdFlags : u32 { # [doc = " `IORING_FIXED_FD_NO_CLOEXEC`"] const NO_CLOEXEC = sys :: IORING_FIXED_FD_NO_CLOEXEC ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
    };
}

macro_359!();