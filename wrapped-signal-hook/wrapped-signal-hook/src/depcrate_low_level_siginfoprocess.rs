// Generated macro for Process (struct)
macro_rules! Depcrate_low_level_siginfoProcess {
() => {
// Module: crate::low_level::siginfo
// Provides: {"Process"}
// Dependencies: {}
# [doc = " Information about process, as presented in the signal metadata."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [non_exhaustive] pub struct Process { # [doc = " The process ID."] pub pid : pid_t , # [doc = " The user owning the process."] pub uid : uid_t , }
};
}
