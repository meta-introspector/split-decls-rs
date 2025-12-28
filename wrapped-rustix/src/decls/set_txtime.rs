macro_rules! deps {
    () => {
        Result!();
        ClockId!();
    };
}

macro_rules! set_txtime {
    () => {
        deps!();
        # [doc = " `setsockopt(fd, SOL_SOCKET, SO_TXTIME)` — Configure transmission timing."] # [cfg (all (target_os = "linux" , feature = "time"))] # [doc (alias = "SO_TXTIME")] pub fn set_txtime < Fd : AsFd > (fd : Fd , clockid : ClockId , flags : TxTimeFlags) -> io :: Result < () > { backend :: net :: sockopt :: set_txtime (fd . as_fd () , clockid , flags) }
    };
}

set_txtime!()