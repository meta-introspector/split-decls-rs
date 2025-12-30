// Generated macro for set_time (function)
macro_rules! Depcrate_unix_linux_processset_time {
() => {
// Module: crate::unix::linux::process
// Provides: {"set_time"}
// Dependencies: {}
pub (crate) fn set_time (p : & mut ProcessInner , utime : u64 , stime : u64) { p . old_utime = p . utime ; p . old_stime = p . stime ; p . utime = utime ; p . stime = stime ; }
};
}
