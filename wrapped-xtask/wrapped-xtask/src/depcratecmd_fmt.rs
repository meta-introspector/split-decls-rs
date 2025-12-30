// Generated macro for cmd_fmt (function)
macro_rules! Depcratecmd_fmt {
() => {
// Module: crate
// Provides: {"cmd_fmt"}
// Dependencies: {}
fn cmd_fmt () -> Result < () , DynError > { cargo (& ["fmt" , "--" , "--check"]) }
};
}
