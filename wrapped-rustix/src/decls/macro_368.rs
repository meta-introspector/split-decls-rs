macro_rules! macro_368 {
    () => {
        bitflags :: bitflags ! { # [doc = " accept flags (`sqe.ioprio`)"] # [repr (transparent)] # [derive (Default , Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct IoringAcceptFlags : u16 { # [doc = " `IORING_ACCEPT_MULTISHOT`"] const MULTISHOT = sys :: IORING_ACCEPT_MULTISHOT as _ ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
    };
}

macro_368!()