// Generated macro for parse_mode (function)
macro_rules! Depcrate_casesparse_mode {
() => {
// Module: crate::cases
// Provides: {"parse_mode"}
// Dependencies: {}
fn parse_mode (var : Option < & std :: ffi :: OsStr >) -> crate :: Mode { if var == Some (std :: ffi :: OsStr :: new ("overwrite")) { crate :: Mode :: Overwrite } else if var == Some (std :: ffi :: OsStr :: new ("dump")) { crate :: Mode :: Dump ("dump" . into ()) } else { crate :: Mode :: Fail } }
};
}
