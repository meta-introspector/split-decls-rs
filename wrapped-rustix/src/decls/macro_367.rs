macro_rules! macro_367 {
    () => {
        bitflags :: bitflags ! { # [doc = " recv/recvmsg flags (`sqe.ioprio`)"] # [repr (transparent)] # [derive (Default , Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct IoringRecvFlags : u16 { # [doc = " `IORING_RECVSEND_POLL_FIRST`"] # [doc = ""] # [doc = " See also [`IoringSendFlags::POLL_FIRST`]."] const POLL_FIRST = sys :: IORING_RECVSEND_POLL_FIRST as _ ; # [doc = " `IORING_RECV_MULTISHOT`"] const MULTISHOT = sys :: IORING_RECV_MULTISHOT as _ ; # [doc = " `IORING_RECVSEND_FIXED_BUF`"] # [doc = ""] # [doc = " See also [`IoringSendFlags::FIXED_BUF`]."] const FIXED_BUF = sys :: IORING_RECVSEND_FIXED_BUF as _ ; # [doc = " `IORING_RECVSEND_BUNDLE`"] # [doc = ""] # [doc = " See also [`IoringSendFlags::BUNDLE`]."] const BUNDLE = sys :: IORING_RECVSEND_BUNDLE as _ ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
    };
}

macro_367!()