// Generated macro for get_compiler_version (function)
macro_rules! Depcrateget_compiler_version {
() => {
// Module: crate
// Provides: {"get_compiler_version"}
// Dependencies: {}
# [must_use] pub fn get_compiler_version () -> Option < String > { let compiler = std :: option_env ! ("RUSTC") . unwrap_or ("rustc") ; get_output (compiler , & ["-V"]) }
};
}
