// Generated macro for macro_599 (macro)
macro_rules! Depcrate_io_uringmacro_599 {
() => {
// Module: crate::io_uring
// Provides: {"macro_599"}
// Dependencies: {}
bitflags :: bitflags ! { # [doc = " `IORING_TIMEOUT_*` and `IORING_LINK_TIMEOUT_UPDATE` flags for use with"] # [doc = " [`io_uring_sqe`]."] # [repr (transparent)] # [derive (Default , Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct IoringTimeoutFlags : u32 { # [doc = " `IORING_TIMEOUT_ABS`"] const ABS = sys :: IORING_TIMEOUT_ABS ; # [doc = " `IORING_TIMEOUT_UPDATE`"] const UPDATE = sys :: IORING_TIMEOUT_UPDATE ; # [doc = " `IORING_TIMEOUT_BOOTTIME`"] const BOOTTIME = sys :: IORING_TIMEOUT_BOOTTIME ; # [doc = " `IORING_TIMEOUT_ETIME_SUCCESS`"] const ETIME_SUCCESS = sys :: IORING_TIMEOUT_ETIME_SUCCESS ; # [doc = " `IORING_TIMEOUT_REALTIME`"] const REALTIME = sys :: IORING_TIMEOUT_REALTIME ; # [doc = " `IORING_TIMEOUT_CLOCK_MASK`"] const CLOCK_MASK = sys :: IORING_TIMEOUT_CLOCK_MASK ; # [doc = " `IORING_TIMEOUT_UPDATE_MASK`"] const UPDATE_MASK = sys :: IORING_TIMEOUT_UPDATE_MASK ; # [doc = " `IORING_LINK_TIMEOUT_UPDATE`"] const LINK_TIMEOUT_UPDATE = sys :: IORING_LINK_TIMEOUT_UPDATE ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
};
}
