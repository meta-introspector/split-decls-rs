// Generated macro for macro_596 (macro)
macro_rules! Depcrate_io_uringmacro_596 {
() => {
// Module: crate::io_uring
// Provides: {"macro_596"}
// Dependencies: {}
bitflags :: bitflags ! { # [doc = " `IOSQE_*` flags for use with [`io_uring_sqe`]."] # [repr (transparent)] # [derive (Default , Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct IoringSqeFlags : u8 { # [doc = " `1 << IOSQE_ASYNC_BIT`"] const ASYNC = 1 << sys :: io_uring_sqe_flags_bit :: IOSQE_ASYNC_BIT as u8 ; # [doc = " `1 << IOSQE_BUFFER_SELECT_BIT`"] const BUFFER_SELECT = 1 << sys :: io_uring_sqe_flags_bit :: IOSQE_BUFFER_SELECT_BIT as u8 ; # [doc = " `1 << IOSQE_FIXED_FILE_BIT`"] const FIXED_FILE = 1 << sys :: io_uring_sqe_flags_bit :: IOSQE_FIXED_FILE_BIT as u8 ; # [doc = " 1 << `IOSQE_IO_DRAIN_BIT`"] const IO_DRAIN = 1 << sys :: io_uring_sqe_flags_bit :: IOSQE_IO_DRAIN_BIT as u8 ; # [doc = " `1 << IOSQE_IO_HARDLINK_BIT`"] const IO_HARDLINK = 1 << sys :: io_uring_sqe_flags_bit :: IOSQE_IO_HARDLINK_BIT as u8 ; # [doc = " `1 << IOSQE_IO_LINK_BIT`"] const IO_LINK = 1 << sys :: io_uring_sqe_flags_bit :: IOSQE_IO_LINK_BIT as u8 ; # [doc = " `1 << IOSQE_CQE_SKIP_SUCCESS_BIT`"] const CQE_SKIP_SUCCESS = 1 << sys :: io_uring_sqe_flags_bit :: IOSQE_CQE_SKIP_SUCCESS_BIT as u8 ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
};
}
