// Generated macro for get_reaper_pids (function)
macro_rules! Depcrate_process_procctlget_reaper_pids {
() => {
// Module: crate::process::procctl
// Provides: {"get_reaper_pids"}
// Dependencies: {}
# [doc = " Get the list of descendants of the specified reaper process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [FreeBSD: `procctl(PROC_REAP_GETPIDS,…)`]"] # [doc = ""] # [doc = " [FreeBSD: `procctl(PROC_REAP_GETPIDS,…)`]: https://man.freebsd.org/cgi/man.cgi?query=procctl&sektion=2"] # [cfg (feature = "alloc")] # [cfg_attr (docsrs , doc (cfg (feature = "alloc")))] pub fn get_reaper_pids (process : ProcSelector) -> io :: Result < Vec < PidInfo > > { const PID_MAX : usize = 99999 ; let mut pids : Vec < procctl_reaper_pidinfo > = vec ! [Default :: default () ; PID_MAX] ; let mut pinfo = procctl_reaper_pids { rp_count : PID_MAX as _ , rp_pad0 : [0 ; 15] , rp_pids : pids . as_mut_slice () . as_mut_ptr () , } ; unsafe { procctl (PROC_REAP_GETPIDS , process , as_mut_ptr (& mut pinfo) . cast ()) ? } ; let mut result = Vec :: new () ; for raw in pids . into_iter () { let flags = PidInfoFlags :: from_bits_retain (raw . pi_flags) ; if ! flags . contains (PidInfoFlags :: VALID) { break ; } result . push (PidInfo { flags , subtree : Pid :: from_raw (raw . pi_subtree) . ok_or (io :: Errno :: RANGE) ? , pid : Pid :: from_raw (raw . pi_pid) . ok_or (io :: Errno :: RANGE) ? , }) ; } Ok (result) }
};
}
