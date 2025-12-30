// Generated macro for macro_587 (macro)
macro_rules! Depcrate_io_uringmacro_587 {
() => {
// Module: crate::io_uring
// Provides: {"macro_587"}
// Dependencies: {}
bitflags :: bitflags ! { # [doc = " `IORING_ENTER_*` flags for use with [`io_uring_enter`]."] # [repr (transparent)] # [derive (Default , Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct IoringEnterFlags : u32 { # [doc = " `IORING_ENTER_GETEVENTS`"] const GETEVENTS = sys :: IORING_ENTER_GETEVENTS ; # [doc = " `IORING_ENTER_SQ_WAKEUP`"] const SQ_WAKEUP = sys :: IORING_ENTER_SQ_WAKEUP ; # [doc = " `IORING_ENTER_SQ_WAIT`"] const SQ_WAIT = sys :: IORING_ENTER_SQ_WAIT ; # [doc = " `IORING_ENTER_EXT_ARG` (since Linux 5.11)"] const EXT_ARG = sys :: IORING_ENTER_EXT_ARG ; # [doc = " `IORING_ENTER_REGISTERED_RING`"] const REGISTERED_RING = sys :: IORING_ENTER_REGISTERED_RING ; # [doc = " `IORING_ENTER_ABS_TIMER` (since Linux 6.12)"] const ABS_TIMER = sys :: IORING_ENTER_ABS_TIMER ; # [doc = " `IORING_ENTER_EXT_ARG_REG` (since Linux 6.12)"] const EXT_ARG_REG = sys :: IORING_ENTER_EXT_ARG_REG ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
};
}
