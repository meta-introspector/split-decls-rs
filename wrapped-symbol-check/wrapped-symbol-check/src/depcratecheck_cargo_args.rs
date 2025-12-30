// Generated macro for check_cargo_args (function)
macro_rules! Depcratecheck_cargo_args {
() => {
// Module: crate
// Provides: {"check_cargo_args"}
// Dependencies: {}
# [doc = " Make sure `--target` isn't passed to avoid confusion (since it should be proivded only once,"] # [doc = " positionally)."] fn check_cargo_args (args : & [& str]) { for arg in args { assert ! (! arg . contains ("--target") , "target must be passed positionally. {USAGE}") ; } }
};
}
