macro_rules! macro_364 {
    () => {
        bitflags :: bitflags ! { # [doc = " `IORING_CQ_*` flags."] # [repr (transparent)] # [derive (Default , Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct IoringCqFlags : u32 { # [doc = " `IORING_CQ_EVENTFD_DISABLED`"] const EVENTFD_DISABLED = sys :: IORING_CQ_EVENTFD_DISABLED ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
    };
}

macro_364!();