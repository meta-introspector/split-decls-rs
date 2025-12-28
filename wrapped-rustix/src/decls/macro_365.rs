macro_rules! macro_365 {
    () => {
        bitflags :: bitflags ! { # [doc = " `IORING_POLL_*` flags."] # [repr (transparent)] # [derive (Default , Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct IoringPollFlags : u32 { # [doc = " `IORING_POLL_ADD_MULTI`"] const ADD_MULTI = sys :: IORING_POLL_ADD_MULTI ; # [doc = " `IORING_POLL_UPDATE_EVENTS`"] const UPDATE_EVENTS = sys :: IORING_POLL_UPDATE_EVENTS ; # [doc = " `IORING_POLL_UPDATE_USER_DATA`"] const UPDATE_USER_DATA = sys :: IORING_POLL_UPDATE_USER_DATA ; # [doc = " `IORING_POLL_ADD_LEVEL`"] const ADD_LEVEL = sys :: IORING_POLL_ADD_LEVEL ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
    };
}

macro_365!()