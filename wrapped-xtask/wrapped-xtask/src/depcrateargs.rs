// Generated macro for Args (struct)
macro_rules! DepcrateArgs {
() => {
// Module: crate
// Provides: {"Args"}
// Dependencies: {}
# [derive (Debug , Parser)] # [command (bin_name = "cargo xtask" , styles = HELP_STYLES)] struct Args { # [command (subcommand)] command : Command , # [command (flatten)] verbosity : Verbosity < InfoLevel > , }
};
}
