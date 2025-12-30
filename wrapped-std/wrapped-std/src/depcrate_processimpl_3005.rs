// Generated macro for impl_3005 (impl)
macro_rules! Depcrate_processimpl_3005 {
() => {
// Module: crate::process
// Provides: {"impl_3005"}
// Dependencies: {}
# [doc = " The default value is one which indicates successful completion."] # [stable (feature = "process_exitstatus_default" , since = "1.73.0")] impl Default for ExitStatus { fn default () -> Self { ExitStatus :: from_inner (imp :: ExitStatus :: default ()) } }
};
}
