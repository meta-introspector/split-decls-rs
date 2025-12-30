// Generated macro for CliOptions (trait)
macro_rules! Depcrate_config_optionsCliOptions {
() => {
// Module: crate::config::options
// Provides: {"CliOptions"}
// Dependencies: {}
# [doc = " Maps client-supplied options to Rustfmt's internals, mostly overriding"] # [doc = " values in a config with values from the command line."] pub trait CliOptions { fn apply_to (self , config : & mut Config) ; # [doc = " It is ok if the returned path doesn't exist or is not canonicalized"] # [doc = " (i.e. the callers are expected to handle such cases)."] fn config_path (& self) -> Option < & Path > ; fn edition (& self) -> Option < Edition > ; fn style_edition (& self) -> Option < StyleEdition > ; fn version (& self) -> Option < Version > ; }
};
}
