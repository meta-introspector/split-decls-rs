// Generated macro for prlimit (function)
macro_rules! Depcrate_process_rlimitprlimit {
() => {
// Module: crate::process::rlimit
// Provides: {"prlimit"}
// Dependencies: {}
# [doc = " `prlimit(pid, resource, new)`—Get and set a process resource limit value."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/prlimit.2.html"] # [cfg (linux_kernel)] # [inline] pub fn prlimit (pid : Option < Pid > , resource : Resource , new : Rlimit) -> io :: Result < Rlimit > { backend :: process :: syscalls :: prlimit (pid , resource , new) }
};
}
