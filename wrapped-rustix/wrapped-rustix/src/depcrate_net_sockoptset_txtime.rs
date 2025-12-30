// Generated macro for set_txtime (function)
macro_rules! Depcrate_net_sockoptset_txtime {
() => {
// Module: crate::net::sockopt
// Provides: {"set_txtime"}
// Dependencies: {}
# [doc = " `setsockopt(fd, SOL_SOCKET, SO_TXTIME)` — Configure transmission timing."] # [cfg (all (target_os = "linux" , feature = "time"))] # [doc (alias = "SO_TXTIME")] pub fn set_txtime < Fd : AsFd > (fd : Fd , clockid : ClockId , flags : TxTimeFlags) -> io :: Result < () > { backend :: net :: sockopt :: set_txtime (fd . as_fd () , clockid , flags) }
};
}
