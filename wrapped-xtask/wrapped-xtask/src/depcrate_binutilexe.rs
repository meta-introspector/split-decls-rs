// Generated macro for exe (function)
macro_rules! Depcrate_binutilexe {
() => {
// Module: crate::binutil
// Provides: {"exe"}
// Dependencies: {}
fn exe (name : & str) -> String { let exe_suffix = std :: env :: consts :: EXE_SUFFIX ; format ! ("llvm-{name}{exe_suffix}") }
};
}
