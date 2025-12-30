// Generated macro for clock_ticks_per_second (function)
macro_rules! Depcrate_param_auxvclock_ticks_per_second {
() => {
// Module: crate::param::auxv
// Provides: {"clock_ticks_per_second"}
// Dependencies: {}
# [doc = " `sysconf(_SC_CLK_TCK)`—Returns the process' clock ticks per second."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/sysconf.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man3/sysconf.3.html"] # [cfg (not (any (target_os = "horizon" , target_os = "vita" , target_os = "wasi")))] # [inline] # [doc (alias = "_SC_CLK_TCK")] pub fn clock_ticks_per_second () -> u64 { backend :: param :: auxv :: clock_ticks_per_second () }
};
}
