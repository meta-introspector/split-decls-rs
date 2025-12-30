// Generated macro for run_cargo_nightly (function)
macro_rules! Depcraterun_cargo_nightly {
() => {
// Module: crate
// Provides: {"run_cargo_nightly"}
// Dependencies: {}
# [doc = " Run a cargo subcommand with the nightly toolchain"] fn run_cargo_nightly (args : Vec < & str >) -> Result < () > { cmd ("cargo" , args) . env_remove ("CARGO") . env ("RUSTUP_TOOLCHAIN" , "nightly") . run_with_trace () ? ; Ok (()) }
};
}
