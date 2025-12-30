// Generated macro for macro_608 (macro)
macro_rules! Depcrate_io_uringmacro_608 {
() => {
// Module: crate::io_uring
// Provides: {"macro_608"}
// Dependencies: {}
bitflags :: bitflags ! { # [doc = " `IORING_SQ_*` flags."] # [repr (transparent)] # [derive (Default , Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct IoringSqFlags : u32 { # [doc = " `IORING_SQ_NEED_WAKEUP`"] const NEED_WAKEUP = sys :: IORING_SQ_NEED_WAKEUP ; # [doc = " `IORING_SQ_CQ_OVERFLOW`"] const CQ_OVERFLOW = sys :: IORING_SQ_CQ_OVERFLOW ; # [doc = " `IORING_SQ_TASKRUN`"] const TASKRUN = sys :: IORING_SQ_TASKRUN ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
};
}
