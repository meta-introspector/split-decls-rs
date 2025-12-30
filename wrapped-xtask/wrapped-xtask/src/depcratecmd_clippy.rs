// Generated macro for cmd_clippy (function)
macro_rules! Depcratecmd_clippy {
() => {
// Module: crate
// Provides: {"cmd_clippy"}
// Dependencies: {}
fn cmd_clippy () -> Result < () , DynError > { cargo (& ["clippy" , "--workspace" , "--features" , "all" , "--all-targets" ,]) }
};
}
