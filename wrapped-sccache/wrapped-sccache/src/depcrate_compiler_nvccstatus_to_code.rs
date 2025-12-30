// Generated macro for status_to_code (function)
macro_rules! Depcrate_compiler_nvccstatus_to_code {
() => {
// Module: crate::compiler::nvcc
// Provides: {"status_to_code"}
// Dependencies: {}
# [cfg (windows)] fn status_to_code (res : process :: ExitStatus) -> ExitStatusValue { if res . success () { 0 as ExitStatusValue } else { res . code () . unwrap_or (1) as ExitStatusValue } }
};
}
