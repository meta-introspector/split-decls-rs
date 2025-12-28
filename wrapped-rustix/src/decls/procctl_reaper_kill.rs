macro_rules! deps {
    () => {
        RawPid!();
    };
}

macro_rules! procctl_reaper_kill {
    () => {
        deps!();
        # [repr (C)] struct procctl_reaper_kill { rk_sig : c_int , rk_flags : c_uint , rk_subtree : RawPid , rk_killed : c_uint , rk_fpid : RawPid , rk_pad0 : [c_uint ; 15] , }
    };
}

procctl_reaper_kill!();