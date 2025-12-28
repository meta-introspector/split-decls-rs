macro_rules! macro_344 {
    () => {
        bitflags :: bitflags ! { # [doc = " `IORING_REGISTER_*` flags for use with [`io_uring_register_with`]."] # [repr (transparent)] # [derive (Default , Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct IoringRegisterFlags : u32 { # [doc = " `IORING_REGISTER_USE_REGISTERED_RING`"] const USE_REGISTERED_RING = sys :: io_uring_register_op :: IORING_REGISTER_USE_REGISTERED_RING as u32 ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
    };
}

macro_344!()