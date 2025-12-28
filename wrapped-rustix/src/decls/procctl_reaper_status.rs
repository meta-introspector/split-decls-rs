macro_rules! deps {
    () => {
        RawPid!();
    };
}

macro_rules! procctl_reaper_status {
    () => {
        deps!();
        # [repr (C)] struct procctl_reaper_status { rs_flags : c_uint , rs_children : c_uint , rs_descendants : c_uint , rs_reaper : RawPid , rs_pid : RawPid , rs_pad0 : [c_uint ; 15] , }
    };
}

procctl_reaper_status!()