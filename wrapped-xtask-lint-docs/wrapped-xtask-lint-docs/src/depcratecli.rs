// Generated macro for cli (function)
macro_rules! Depcratecli {
() => {
// Module: crate
// Provides: {"cli"}
// Dependencies: {}
fn cli () -> clap :: Command { clap :: Command :: new ("xtask-lint-docs") . arg (flag ("check" , "Check that the docs are up-to-date")) }
};
}
