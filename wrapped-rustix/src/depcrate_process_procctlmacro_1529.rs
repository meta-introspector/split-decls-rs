// Generated macro for macro_1529 (macro)
macro_rules! Depcrate_process_procctlmacro_1529 {
() => {
// Module: crate::process::procctl
// Provides: {"macro_1529"}
// Dependencies: {}
bitflags ! { # [doc = " `REAPER_STATUS_*`"] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct ReaperStatusFlags : c_uint { # [doc = " The process has acquired reaper status."] const OWNED = 1 ; # [doc = " The process is the root of the reaper tree ([`Pid::INIT`])."] const REALINIT = 2 ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
};
}
