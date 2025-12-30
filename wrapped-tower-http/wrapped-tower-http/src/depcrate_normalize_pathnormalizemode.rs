// Generated macro for NormalizeMode (enum)
macro_rules! Depcrate_normalize_pathNormalizeMode {
() => {
// Module: crate::normalize_path
// Provides: {"NormalizeMode"}
// Dependencies: {}
# [doc = " Different modes of normalizing paths"] # [derive (Debug , Copy , Clone)] enum NormalizeMode { # [doc = " Normalizes paths by trimming the trailing slashes, e.g. /foo/ -> /foo"] Trim , # [doc = " Normalizes paths by appending trailing slash, e.g. /foo -> /foo/"] Append , }
};
}
