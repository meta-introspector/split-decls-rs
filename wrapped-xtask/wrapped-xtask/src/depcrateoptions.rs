// Generated macro for Options (struct)
macro_rules! DepcrateOptions {
() => {
// Module: crate
// Provides: {"Options"}
// Dependencies: {}
# [derive (Debug , Parser)] struct Options { # [command (subcommand)] cmd : TestCommand , # [doc = " Treat compiler warnings as errors (`RUSTFLAGS=\"--deny warnings\"`)"] # [arg (long , short)] deny_warnings : bool , # [doc = " Keep target toolchains that were installed as dependency"] # [arg (long , short)] keep_targets : bool , }
};
}
