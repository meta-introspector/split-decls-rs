macro_rules! macro_659 {
    () => {
        # [cfg (all (target_os = "linux" , feature = "time"))] bitflags ! { # [doc = " Flags for use with [`set_txtime`]."] # [doc = ""] # [doc = " [`set_txtime`]: crate::net::sockopt::set_txtime"] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct TxTimeFlags : u32 { # [doc = " `SOF_TXTIME_DEADLINE_MODE`"] const DEADLINE_MODE = bitcast ! (c :: SOF_TXTIME_DEADLINE_MODE) ; # [doc = " `SOF_TXTIME_REPORT_ERRORS`"] const REPORT_ERRORS = bitcast ! (c :: SOF_TXTIME_REPORT_ERRORS) ; } }
    };
}

macro_659!()