// Generated macro for TracingStatus (enum)
macro_rules! Depcrate_process_procctlTracingStatus {
() => {
// Module: crate::process::procctl
// Provides: {"TracingStatus"}
// Dependencies: {}
# [doc = " Tracing status as returned by [`trace_status`]."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub enum TracingStatus { # [doc = " Tracing is disabled for the process."] NotTraceble , # [doc = " Tracing is not disabled for the process, but not debugger/tracer is"] # [doc = " attached."] Tracable , # [doc = " The process is being traced by the process whose pid is stored in the"] # [doc = " first component of this variant."] BeingTraced (Pid) , }
};
}
