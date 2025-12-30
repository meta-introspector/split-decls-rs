// Generated macro for procctl_reaper_status (struct)
macro_rules! Depcrate_process_procctlprocctl_reaper_status {
() => {
// Module: crate::process::procctl
// Provides: {"procctl_reaper_status"}
// Dependencies: {}
# [repr (C)] struct procctl_reaper_status { rs_flags : c_uint , rs_children : c_uint , rs_descendants : c_uint , rs_reaper : RawPid , rs_pid : RawPid , rs_pad0 : [c_uint ; 15] , }
};
}
