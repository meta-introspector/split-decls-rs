// Generated macro for FileName (enum)
macro_rules! Depcrate_config_file_linesFileName {
() => {
// Module: crate::config::file_lines
// Provides: {"FileName"}
// Dependencies: {}
# [doc = " Defines the name of an input - either a file or stdin."] # [derive (Clone , Debug , Eq , PartialEq , Hash , Ord , PartialOrd)] pub enum FileName { Real (PathBuf) , Stdin , }
};
}
