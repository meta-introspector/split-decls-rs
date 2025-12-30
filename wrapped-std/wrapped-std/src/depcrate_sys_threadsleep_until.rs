// Generated macro for sleep_until (function)
macro_rules! Depcrate_sys_threadsleep_until {
() => {
// Module: crate::sys::thread
// Provides: {"sleep_until"}
// Dependencies: {}
# [cfg (not (any (target_os = "freebsd" , target_os = "netbsd" , target_os = "linux" , target_os = "android" , target_os = "solaris" , target_os = "illumos" , target_os = "dragonfly" , target_os = "hurd" , target_os = "fuchsia" , target_os = "vxworks" , all (target_os = "wasi" , target_env = "p2") ,)))] pub fn sleep_until (deadline : crate :: time :: Instant) { use crate :: time :: Instant ; let now = Instant :: now () ; if let Some (delay) = deadline . checked_duration_since (now) { sleep (delay) ; } }
};
}
