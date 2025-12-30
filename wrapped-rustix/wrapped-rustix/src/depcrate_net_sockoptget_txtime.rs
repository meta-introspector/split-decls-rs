// Generated macro for get_txtime (function)
macro_rules! Depcrate_net_sockoptget_txtime {
() => {
// Module: crate::net::sockopt
// Provides: {"get_txtime"}
// Dependencies: {}
# [doc = " `getsockopt(fd, SOL_SOCKET, SO_TXTIME)` — Get transmission timing configuration."] # [cfg (all (target_os = "linux" , feature = "time"))] # [doc (alias = "SO_TXTIME")] pub fn get_txtime < Fd : AsFd > (fd : Fd) -> io :: Result < (ClockId , TxTimeFlags) > { backend :: net :: sockopt :: get_txtime (fd . as_fd ()) }
};
}
