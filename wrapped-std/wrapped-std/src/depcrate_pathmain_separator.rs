// Generated macro for MAIN_SEPARATOR (const)
macro_rules! Depcrate_pathMAIN_SEPARATOR {
() => {
// Module: crate::path
// Provides: {"MAIN_SEPARATOR"}
// Dependencies: {}
# [doc = " The primary separator of path components for the current platform."] # [doc = ""] # [doc = " For example, `/` on Unix and `\\` on Windows."] # [stable (feature = "rust1" , since = "1.0.0")] # [cfg_attr (not (test) , rustc_diagnostic_item = "path_main_separator")] pub const MAIN_SEPARATOR : char = crate :: sys :: path :: MAIN_SEP ;
};
}
