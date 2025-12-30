// Generated macro for run_cargo (function)
macro_rules! Depcraterun_cargo {
() => {
// Module: crate
// Provides: {"run_cargo"}
// Dependencies: {}
# [doc = " Run a cargo subcommand with the default toolchain"] fn run_cargo (args : Vec < & str >) -> Result < () > { cmd ("cargo" , args) . run_with_trace () ? ; Ok (()) }
};
}
