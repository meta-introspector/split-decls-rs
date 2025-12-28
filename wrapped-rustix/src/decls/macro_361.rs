macro_rules! macro_361 {
    () => {
        bitflags :: bitflags ! { # [doc = " `IO_URING_OP_*` flags for use with [`io_uring_probe_op`]."] # [repr (transparent)] # [derive (Default , Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct IoringOpFlags : u16 { # [doc = " `IO_URING_OP_SUPPORTED`"] const SUPPORTED = sys :: IO_URING_OP_SUPPORTED as _ ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
    };
}

macro_361!()