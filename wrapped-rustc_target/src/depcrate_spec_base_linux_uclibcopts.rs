// Generated macro for opts (function)
macro_rules! Depcrate_spec_base_linux_uclibcopts {
() => {
// Module: crate::spec::base::linux_uclibc
// Provides: {"opts"}
// Dependencies: {}
pub (crate) fn opts () -> TargetOptions { TargetOptions { env : "uclibc" . into () , .. base :: linux :: opts () } }
};
}
