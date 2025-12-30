// Generated macro for macro_607 (macro)
macro_rules! Depcrate_io_uringmacro_607 {
() => {
// Module: crate::io_uring
// Provides: {"macro_607"}
// Dependencies: {}
bitflags :: bitflags ! { # [doc = " `IORING_RSRC_*` flags for use with [`io_uring_rsrc_register`]."] # [repr (transparent)] # [derive (Default , Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct IoringRsrcFlags : u32 { # [doc = " `IORING_RSRC_REGISTER_SPARSE`"] const REGISTER_SPARSE = sys :: IORING_RSRC_REGISTER_SPARSE as _ ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
};
}
