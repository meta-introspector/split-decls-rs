// Generated macro for cmd_semver (function)
macro_rules! Depcratecmd_semver {
() => {
// Module: crate
// Provides: {"cmd_semver"}
// Dependencies: {}
fn cmd_semver () -> Result < () , DynError > { cargo (& ["semver-checks" , "--only-explicit-features" , "--features" , "all" , "-p" , "object" ,]) }
};
}
