// Generated macro for macro_606 (macro)
macro_rules! Depcrate_io_uringmacro_606 {
() => {
// Module: crate::io_uring
// Provides: {"macro_606"}
// Dependencies: {}
bitflags :: bitflags ! { # [doc = " `IO_URING_OP_*` flags for use with [`io_uring_probe_op`]."] # [repr (transparent)] # [derive (Default , Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct IoringOpFlags : u16 { # [doc = " `IO_URING_OP_SUPPORTED`"] const SUPPORTED = sys :: IO_URING_OP_SUPPORTED as _ ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
};
}
