// Generated macro for PidInfo (struct)
macro_rules! Depcrate_process_procctlPidInfo {
() => {
// Module: crate::process::procctl
// Provides: {"PidInfo"}
// Dependencies: {}
# [doc = " A child process of a reaper."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct PidInfo { # [doc = " The flags of the process."] pub flags : PidInfoFlags , # [doc = " The pid of the process."] pub pid : Pid , # [doc = " The pid of the child of the reaper which is the (grand-…)parent of the"] # [doc = " process."] pub subtree : Pid , }
};
}
