// Generated macro for MatchArmLeadingPipe (enum)
macro_rules! Depcrate_config_optionsMatchArmLeadingPipe {
() => {
// Module: crate::config::options
// Provides: {"MatchArmLeadingPipe"}
// Dependencies: {}
# [doc = " Controls how rustfmt should handle leading pipes on match arms."] # [config_type] pub enum MatchArmLeadingPipe { # [doc = " Place leading pipes on all match arms"] Always , # [doc = " Never emit leading pipes on match arms"] Never , # [doc = " Preserve any existing leading pipes"] Preserve , }
};
}
