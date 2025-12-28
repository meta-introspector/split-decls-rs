macro_rules! deps {
    () => {
        Result!();
        ClockId!();
    };
}

macro_rules! get_txtime {
    () => {
        deps!();
        # [doc = " `getsockopt(fd, SOL_SOCKET, SO_TXTIME)` — Get transmission timing configuration."] # [cfg (all (target_os = "linux" , feature = "time"))] # [doc (alias = "SO_TXTIME")] pub fn get_txtime < Fd : AsFd > (fd : Fd) -> io :: Result < (ClockId , TxTimeFlags) > { backend :: net :: sockopt :: get_txtime (fd . as_fd ()) }
    };
}

get_txtime!();