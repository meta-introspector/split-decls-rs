macro_rules! macro_355 {
    () => {
        bitflags :: bitflags ! { # [doc = " `SPLICE_F_*` flags for use with [`io_uring_sqe`]."] # [repr (transparent)] # [derive (Default , Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct SpliceFlags : u32 { # [doc = " `SPLICE_F_FD_IN_FIXED`"] const FD_IN_FIXED = sys :: SPLICE_F_FD_IN_FIXED ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
    };
}

macro_355!();