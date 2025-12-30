// Generated macro for Pid (struct)
macro_rules! Depcrate_pidPid {
() => {
// Module: crate::pid
// Provides: {"Pid"}
// Dependencies: {}
# [doc = " `pid_t`—A non-zero Unix process ID."] # [doc = ""] # [doc = " This is a pid, and not a pidfd. It is not a file descriptor, and the"] # [doc = " process it refers to could disappear at any time and be replaced by"] # [doc = " another, unrelated, process."] # [doc = ""] # [doc = " On Linux, `Pid` values are also used to identify threads."] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Debug , Hash)] pub struct Pid (NonZeroI32) ;
};
}
