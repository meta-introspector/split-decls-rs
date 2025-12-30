// Generated macro for procctl_reaper_pidinfo (struct)
macro_rules! Depcrate_process_procctlprocctl_reaper_pidinfo {
() => {
// Module: crate::process::procctl
// Provides: {"procctl_reaper_pidinfo"}
// Dependencies: {}
# [repr (C)] # [derive (Default , Clone)] struct procctl_reaper_pidinfo { pi_pid : RawPid , pi_subtree : RawPid , pi_flags : c_uint , pi_pad0 : [c_uint ; 15] , }
};
}
