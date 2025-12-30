// Generated macro for macro_1540 (macro)
macro_rules! Depcrate_process_procctlmacro_1540 {
() => {
// Module: crate::process::procctl
// Provides: {"macro_1540"}
// Dependencies: {}
bitflags ! { # [doc = " `REAPER_KILL_*`"] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] struct KillFlags : c_uint { const CHILDREN = 1 ; const SUBTREE = 2 ; } }
};
}
