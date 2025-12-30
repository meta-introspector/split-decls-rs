// Generated macro for run_build_and_check (function)
macro_rules! Depcraterun_build_and_check {
() => {
// Module: crate
// Provides: {"run_build_and_check"}
// Dependencies: {}
fn run_build_and_check (target : & str , args : & [& str]) { for arg in args { assert ! (! arg . contains ("--target") , "target must be passed positionally. {USAGE}") ; } let paths = exec_cargo_with_args (target , args) ; check_paths (& paths) ; }
};
}
