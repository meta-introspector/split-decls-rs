// Generated macro for Density (enum)
macro_rules! Depcrate_config_optionsDensity {
() => {
// Module: crate::config::options
// Provides: {"Density"}
// Dependencies: {}
# [config_type] # [doc = " How to place a list-like items."] # [doc = " FIXME: Issue-3581: this should be renamed to ItemsLayout when publishing 2.0"] pub enum Density { # [doc = " Fit as much on one line as possible."] Compressed , # [doc = " Items are placed horizontally if sufficient space, vertically otherwise."] Tall , # [doc = " Place every item on a separate line."] Vertical , }
};
}
