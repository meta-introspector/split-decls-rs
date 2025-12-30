// Generated macro for build (function)
macro_rules! Depcrate_commandsbuild {
() => {
// Module: crate::commands
// Provides: {"build"}
// Dependencies: {}
# [doc = " Build the project"] fn build () -> Result < () > { run_cargo (vec ! ["build" , "--all-targets" , "--all-features"]) }
};
}
