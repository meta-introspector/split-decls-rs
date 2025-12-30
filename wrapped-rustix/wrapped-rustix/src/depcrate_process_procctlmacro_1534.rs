// Generated macro for macro_1534 (macro)
macro_rules! Depcrate_process_procctlmacro_1534 {
() => {
// Module: crate::process::procctl
// Provides: {"macro_1534"}
// Dependencies: {}
bitflags ! { # [doc = " `REAPER_PIDINFO_*`"] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct PidInfoFlags : c_uint { # [doc = " This structure was filled by the kernel."] const VALID = 1 ; # [doc = " The pid field identifies a direct child of the reaper."] const CHILD = 2 ; # [doc = " The reported process is itself a reaper. Descendants of a"] # [doc = " subordinate reaper are not reported."] const REAPER = 4 ; # [doc = " The reported process is in the zombie state."] const ZOMBIE = 8 ; # [doc = " The reported process is stopped by"] # [doc = " [`Signal::Stop`]/[`Signal::Tstp`]."] const STOPPED = 16 ; # [doc = " The reported process is in the process of exiting."] const EXITING = 32 ; } }
};
}
