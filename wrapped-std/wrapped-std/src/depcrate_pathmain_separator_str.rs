// Generated macro for MAIN_SEPARATOR_STR (const)
macro_rules! Depcrate_pathMAIN_SEPARATOR_STR {
() => {
// Module: crate::path
// Provides: {"MAIN_SEPARATOR_STR"}
// Dependencies: {}
# [doc = " The primary separator of path components for the current platform."] # [doc = ""] # [doc = " For example, `/` on Unix and `\\` on Windows."] # [stable (feature = "main_separator_str" , since = "1.68.0")] pub const MAIN_SEPARATOR_STR : & str = crate :: sys :: path :: MAIN_SEP_STR ;
};
}
