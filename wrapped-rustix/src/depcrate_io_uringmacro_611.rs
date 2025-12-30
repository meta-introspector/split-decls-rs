// Generated macro for macro_611 (macro)
macro_rules! Depcrate_io_uringmacro_611 {
() => {
// Module: crate::io_uring
// Provides: {"macro_611"}
// Dependencies: {}
bitflags :: bitflags ! { # [doc = " send/sendmsg flags (`sqe.ioprio`)"] # [repr (transparent)] # [derive (Default , Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct IoringSendFlags : u16 { # [doc = " `IORING_RECVSEND_POLL_FIRST`."] # [doc = ""] # [doc = " See also [`IoringRecvFlags::POLL_FIRST`]."] const POLL_FIRST = sys :: IORING_RECVSEND_POLL_FIRST as _ ; # [doc = " `IORING_RECVSEND_FIXED_BUF`"] # [doc = ""] # [doc = " See also [`IoringRecvFlags::FIXED_BUF`]."] const FIXED_BUF = sys :: IORING_RECVSEND_FIXED_BUF as _ ; # [doc = " `IORING_SEND_ZC_REPORT_USAGE` (since Linux 6.2)"] const ZC_REPORT_USAGE = sys :: IORING_SEND_ZC_REPORT_USAGE as _ ; # [doc = " `IORING_RECVSEND_BUNDLE`"] # [doc = ""] # [doc = " See also [`IoringRecvFlags::BUNDLE`]."] const BUNDLE = sys :: IORING_RECVSEND_BUNDLE as _ ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
};
}
