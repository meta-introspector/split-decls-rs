macro_rules! deps {
    () => {
        ICause!();
    };
}

macro_rules! other_78 {
    () => {
        deps!();
        extern "C" { fn sighook_signal_cause (info : & siginfo_t) -> ICause ; fn sighook_signal_pid (info : & siginfo_t) -> pid_t ; fn sighook_signal_uid (info : & siginfo_t) -> uid_t ; }
    };
}

other_78!();