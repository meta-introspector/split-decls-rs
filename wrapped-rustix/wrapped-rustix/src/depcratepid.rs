// Generated macro for pid (module)
macro_rules! Depcratepid {
() => {
// Module: crate
// Provides: {"pid"}
// Dependencies: {}
# [cfg (not (any (windows , target_os = "wasi")))] # [cfg (any (feature = "process" , feature = "runtime" , feature = "termios" , feature = "thread" , all (bsd , feature = "event") , all (linux_kernel , feature = "net")))] mod pid ;
};
}
