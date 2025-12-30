// Generated macro for macro_609 (macro)
macro_rules! Depcrate_io_uringmacro_609 {
() => {
// Module: crate::io_uring
// Provides: {"macro_609"}
// Dependencies: {}
bitflags :: bitflags ! { # [doc = " `IORING_CQ_*` flags."] # [repr (transparent)] # [derive (Default , Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct IoringCqFlags : u32 { # [doc = " `IORING_CQ_EVENTFD_DISABLED`"] const EVENTFD_DISABLED = sys :: IORING_CQ_EVENTFD_DISABLED ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
};
}
