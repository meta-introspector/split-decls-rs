// Generated macro for IgnoreList (struct)
macro_rules! Depcrate_config_optionsIgnoreList {
() => {
// Module: crate::config::options
// Provides: {"IgnoreList"}
// Dependencies: {}
# [doc = " A set of directories, files and modules that rustfmt should ignore."] # [derive (Default , Clone , Debug , PartialEq)] pub struct IgnoreList { # [doc = " A set of path specified in rustfmt.toml."] path_set : HashSet < PathBuf > , # [doc = " A path to rustfmt.toml."] rustfmt_toml_path : PathBuf , }
};
}
