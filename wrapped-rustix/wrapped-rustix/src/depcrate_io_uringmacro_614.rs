// Generated macro for macro_614 (macro)
macro_rules! Depcrate_io_uringmacro_614 {
() => {
// Module: crate::io_uring
// Provides: {"macro_614"}
// Dependencies: {}
bitflags :: bitflags ! { # [doc = " recvmsg out flags"] # [repr (transparent)] # [derive (Default , Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct RecvmsgOutFlags : u32 { # [doc = " `MSG_EOR`"] const EOR = net :: MSG_EOR ; # [doc = " `MSG_TRUNC`"] const TRUNC = net :: MSG_TRUNC ; # [doc = " `MSG_CTRUNC`"] const CTRUNC = net :: MSG_CTRUNC ; # [doc = " `MSG_OOB`"] const OOB = net :: MSG_OOB ; # [doc = " `MSG_ERRQUEUE`"] const ERRQUEUE = net :: MSG_ERRQUEUE ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
};
}
