macro_rules! procctl_reaper_pids {
    () => {
        # [repr (C)] struct procctl_reaper_pids { rp_count : c_uint , rp_pad0 : [c_uint ; 15] , rp_pids : * mut procctl_reaper_pidinfo , }
    };
}

procctl_reaper_pids!()