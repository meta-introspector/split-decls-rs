// Generated macro for macro_589 (macro)
macro_rules! Depcrate_io_uringmacro_589 {
() => {
// Module: crate::io_uring
// Provides: {"macro_589"}
// Dependencies: {}
bitflags :: bitflags ! { # [doc = " `IORING_REGISTER_*` flags for use with [`io_uring_register_with`]."] # [repr (transparent)] # [derive (Default , Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct IoringRegisterFlags : u32 { # [doc = " `IORING_REGISTER_USE_REGISTERED_RING`"] const USE_REGISTERED_RING = sys :: io_uring_register_op :: IORING_REGISTER_USE_REGISTERED_RING as u32 ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
};
}
