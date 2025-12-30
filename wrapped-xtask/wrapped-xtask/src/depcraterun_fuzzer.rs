// Generated macro for run_fuzzer (function)
macro_rules! Depcraterun_fuzzer {
() => {
// Module: crate
// Provides: {"run_fuzzer"}
// Dependencies: {}
fn run_fuzzer (sh : & Shell) -> anyhow :: Result < () > { let _d = sh . push_dir ("./crates/syntax") ; let _e = sh . push_env ("RUSTUP_TOOLCHAIN" , "nightly") ; if cmd ! (sh , "cargo fuzz --help") . read () . is_err () { cmd ! (sh , "cargo install cargo-fuzz") . run () ? ; } ; let out = cmd ! (sh , "rustc --version") . read () ? ; if ! out . contains ("nightly") { bail ! ("fuzz tests require nightly rustc") } cmd ! (sh , "cargo fuzz run parser") . run () ? ; Ok (()) }
};
}
