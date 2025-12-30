// Generated macro for FileLinesError (enum)
macro_rules! Depcrate_config_file_linesFileLinesError {
() => {
// Module: crate::config::file_lines
// Provides: {"FileLinesError"}
// Dependencies: {}
# [derive (Error , Debug)] pub enum FileLinesError { # [error ("{0}")] Json (json :: Error) , # [error ("Can't canonicalize {0}")] CannotCanonicalize (FileName) , }
};
}
