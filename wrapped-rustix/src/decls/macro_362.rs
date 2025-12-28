macro_rules! macro_362 {
    () => {
        bitflags :: bitflags ! { # [doc = " `IORING_RSRC_*` flags for use with [`io_uring_rsrc_register`]."] # [repr (transparent)] # [derive (Default , Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct IoringRsrcFlags : u32 { # [doc = " `IORING_RSRC_REGISTER_SPARSE`"] const REGISTER_SPARSE = sys :: IORING_RSRC_REGISTER_SPARSE as _ ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
    };
}

macro_362!();