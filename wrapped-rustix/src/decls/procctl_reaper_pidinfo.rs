macro_rules! deps {
    () => {
        RawPid!();
    };
}

macro_rules! procctl_reaper_pidinfo {
    () => {
        deps!();
        # [repr (C)] # [derive (Default , Clone)] struct procctl_reaper_pidinfo { pi_pid : RawPid , pi_subtree : RawPid , pi_flags : c_uint , pi_pad0 : [c_uint ; 15] , }
    };
}

procctl_reaper_pidinfo!();