// Generated macro for PTracer (enum)
macro_rules! Depcrate_process_prctlPTracer {
() => {
// Module: crate::process::prctl
// Provides: {"PTracer"}
// Dependencies: {}
# [doc = " Process ptracer."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub enum PTracer { # [doc = " None."] None , # [doc = " Disable `ptrace` restrictions for the calling process."] Any , # [doc = " Specific process."] ProcessID (Pid) , }
};
}
